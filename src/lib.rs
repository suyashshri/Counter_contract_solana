use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info,AccountInfo},
    entrypoint,
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

#[derive(BorshSerialize,BorshDeserialize)]
enum Instruction {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshSerialize,BorshDeserialize)]
struct Counter{
    count : u32
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instructuon_data: &[u8]
)->ProgramResult {
    let account = next_account_info(&mut account_info.iter())?;
    let mut counter = Counter::try_from_slice(&mut account.data.borrow())?;

    let action = Instruction::try_from_slice(instructuon_data)?;

    match action {
        Instruction::Increment(value) => {
            counter.count += value
        }
        Instruction::Decrement(value) => {
            counter.count -= value
        }
    }

    counter.serialize(&mut *account.data.borrow_mut());

    msg!("Counter updated to {}",counter.count);

    Ok(())
}