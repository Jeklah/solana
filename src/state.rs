use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_arrary_refs};

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        const LEN: usize = 105;
        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Escrow::LEN];
            let (
                is_initialized,
                initializer_pubkey,
                temp_token_account_pubkey,
                initializer_token_to_receive_account_pubkey,
                expected_amount,
            ) = array_refs![src, 1, 32, 32, 32, 8];
            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            Ok(Escrow {
                is_initialized,
                initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
                temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
                initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),
                expected_amount: u64::from_le_bytes(*expected_amount),
            })
        }
    }
}

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
}
