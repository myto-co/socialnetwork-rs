use anchor_lang::prelude::*;

#[error_code]
pub enum TiktaalikError {
    #[msg("Username has already been taken")]
    UsernameTaken,
    #[msg("Account is not registered")]
    ProfileNotFound,
    #[msg("Content hash is empty")]
    EmptyContent,
    #[msg("User authority does not match")]
    AuthorityDoesNotMatch,
    #[msg("Poster account does not match with creator")]
    InvalidPoster,
    #[msg("SOL transfer failed")]
    SolTransferFailed
}