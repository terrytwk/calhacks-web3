#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct EscrowAccount {
    pub model_hash: String,
    pub amount: u64,
    pub payee: Pubkey,
    pub owner: Pubkey,
}

impl<'info, 'entrypoint> EscrowAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedEscrowAccount<'info, 'entrypoint>> {
        let model_hash = account.model_hash.clone();
        let amount = account.amount;
        let payee = account.payee.clone();
        let owner = account.owner.clone();

        Mutable::new(LoadedEscrowAccount {
            __account__: account,
            __programs__: programs_map,
            model_hash,
            amount,
            payee,
            owner,
        })
    }

    pub fn store(loaded: Mutable<LoadedEscrowAccount>) {
        let mut loaded = loaded.borrow_mut();
        let model_hash = loaded.model_hash.clone();

        loaded.__account__.model_hash = model_hash;

        let amount = loaded.amount;

        loaded.__account__.amount = amount;

        let payee = loaded.payee.clone();

        loaded.__account__.payee = payee;

        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;
    }
}

#[derive(Debug)]
pub struct LoadedEscrowAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, EscrowAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub model_hash: String,
    pub amount: u64,
    pub payee: Pubkey,
    pub owner: Pubkey,
}

pub fn verify_model_handler<'info>(
    mut escrow_account: Mutable<LoadedEscrowAccount<'info, '_>>,
    mut payee: UncheckedAccount<'info>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(signer.key() == escrow_account.borrow().owner) {
        panic!("Not owner");
    }

    if !(escrow_account.borrow().payee == payee.key()) {
        panic!("Incorrect payee");
    }

    {
        let amount = escrow_account.borrow().amount;

        **escrow_account
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **payee.to_account_info().try_borrow_mut_lamports().unwrap() += amount;
    };
}

pub fn init_escrow_account_handler<'info>(
    mut new_escrow_account: Empty<Mutable<LoadedEscrowAccount<'info, '_>>>,
    mut signer: SeahorseSigner<'info, '_>,
    mut payee: Pubkey,
    mut amount: u64,
    mut model_hash: String,
    mut owner: Pubkey,
) -> () {
    let mut new_escrow_account = new_escrow_account.account.clone();

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &signer.key(),
            &new_escrow_account.borrow().__account__.key(),
            amount,
        ),
        &[
            signer.to_account_info(),
            new_escrow_account.borrow().__account__.to_account_info(),
            signer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();

    assign!(new_escrow_account.borrow_mut().model_hash, model_hash);

    assign!(new_escrow_account.borrow_mut().payee, payee);

    assign!(new_escrow_account.borrow_mut().amount, amount);

    assign!(new_escrow_account.borrow_mut().owner, owner);
}
