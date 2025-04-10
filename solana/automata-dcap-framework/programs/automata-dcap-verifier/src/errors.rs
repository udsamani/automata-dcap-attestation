use anchor_lang::prelude::*;

#[error_code]
pub enum DcapVerifierError {
    #[msg("Invalid buffer owner")]
    InvalidOwner,

    #[msg("Buffer already complete")]
    BufferAlreadyComplete,

    #[msg("Invalid chunk index")]
    InvalidChunkIndex,

    #[msg("Chunk out of bounds")]
    ChunkOutOfBounds,

    #[msg("Incomplete quote")]
    IncompleteQuote,

    #[msg("Invalid quote")]
    InvalidQuote,

    #[msg("Invalid Secp256r1 instruction")]
    InvalidSecp256r1Instruction,
}
