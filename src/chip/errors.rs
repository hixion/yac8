use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("Address of out memory boundary")]
    AddressBoundaryError,
    #[error("Invalid Opcode operand")]
    InvalidOpcode,
}
