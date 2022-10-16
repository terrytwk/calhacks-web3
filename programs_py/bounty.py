# bounty
# Built with Seahorse v0.2.1

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class EscrowAccount(Account):
    model_hash: str
    amount: u64
    payee: Pubkey
    owner: Pubkey

@instruction
def init_escrow_account(
  new_escrow_account: Empty[EscrowAccount],
  signer: Signer,
  payee: Pubkey,
  amount: u64,
  model_hash: str,
  owner: Pubkey,
):
    new_escrow_account = new_escrow_account.init(
        payer = signer,
        seeds = ['escrow', model_hash]
    )
    signer.transfer_lamports(new_escrow_account, amount)
    new_escrow_account.model_hash = model_hash
    new_escrow_account.payee = payee
    new_escrow_account.amount = amount
    new_escrow_account.owner = owner

@instruction
def verify_model(
    escrow_account: EscrowAccount,
    payee: UncheckedAccount,
    signer: Signer,
):
    assert signer.key() == escrow_account.owner, "Not owner"
    assert escrow_account.payee == payee.key(), "Incorrect payee"

    escrow_account.transfer_lamports(payee, escrow_account.amount)