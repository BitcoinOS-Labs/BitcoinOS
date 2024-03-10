use candid::CandidType;

#[derive(Debug, CandidType)]
pub enum Error {
    AlreadyExists,
    WriteError,
    Unknown,
}
