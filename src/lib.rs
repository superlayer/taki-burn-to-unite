use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  msg,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
};

use spl_token::instruction::burn;
use std::convert::TryFrom;
use std::str::FromStr;
use spl_associated_token_account::get_associated_token_address;

// Define a struct to represent the token account data
#[derive(Debug)]
struct TokenAccount {
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
}

entrypoint!(process_instruction);

fn process_instruction(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> Result<(), ProgramError> {
  // Extract accounts
  let accounts_iter: &mut std::slice::Iter<AccountInfo> = &mut accounts.iter();
  let owner_account = next_account_info(accounts_iter)?;
  
  // Check if the owner is the signer
  if !owner_account.is_signer {
    msg!("Owner should be a signer");
    return Err(ProgramError::MissingRequiredSignature);
  }

  // Get the associated token account address for the owner and the mint
  let mint_address = "Taki7fi3Zicv7Du1xNAWLaf6mRK7ikdn77HeGzgwvo4"; // Your token mint address
  let mint_pubkey = Pubkey::from_str(mint_address).unwrap();

  let token_account_address = get_associated_token_address(
    owner_account.key,
    &mint_pubkey,
  );

  let token_account = next_account_info(accounts_iter)?;
  if token_account.key != &token_account_address {
      msg!("Token account address mismatch, expecting {} and received {}", token_account_address.to_string(), token_account.key.to_string());
      return Err(ProgramError::InvalidAccountData);
  }

  let mint_account = next_account_info(accounts_iter)?;
  if mint_account.key != &mint_pubkey {
      msg!("Mint account address mismatch");
      return Err(ProgramError::InvalidAccountData);
  }

  msg!("Deserializing token account data from account {}", token_account_address.to_string());
  let token_account_data = TokenAccount::deserialize(&token_account.data.borrow())?;

  // Process the amount to burn
  msg!("Amount to burn: {}", token_account_data.amount);

  // Create the burn instruction
  let ix = burn(
      &spl_token::id(),
      &token_account_address,
      &mint_pubkey,
      owner_account.key,
      &[],
      token_account_data.amount,
  )?;

  // Invoke the burn instruction
  invoke(
      &ix,
      &[
          owner_account.clone(),
          token_account.clone(),
          mint_account.clone(),
      ],
  )?;

  msg!("Burned {} tokens from account {}", token_account_data.amount, token_account_address);

  Ok(())
}


impl TokenAccount {
  // Deserialize token account data from a byte slice
  fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
      // Ensure that the input slice has the minimum required length
      if input.len() < 32 + 32 + 8 {
          return Err(ProgramError::InvalidAccountData);
      }

      // Deserialize the fields from the byte slice
      let mut offset = 0;
      let mut mint_bytes = [0u8; 32];
      mint_bytes.copy_from_slice(&input[offset..offset + 32]);
      let mint = Pubkey::new_from_array(mint_bytes);
      offset += 32;

      let mut owner_bytes = [0u8; 32];
      owner_bytes.copy_from_slice(&input[offset..offset + 32]);
      let owner = Pubkey::new_from_array(owner_bytes);
      offset += 32;

      let amount = u64::from_le_bytes(<[u8; 8]>::try_from(&input[offset..offset + 8]).unwrap());

      Ok(Self { mint, owner, amount })
  }
}