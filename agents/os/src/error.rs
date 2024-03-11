use candid::CandidType;

#[derive(Debug, CandidType)]
pub enum Error {
    AlreadyExists,
    CreateCanisterFailed(String),
    WriteError,
    Unknown,
}
