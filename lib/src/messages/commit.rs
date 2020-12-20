use bytes::*;
use neo4rs_macros::BoltStruct;

pub const MARKER: u8 = 0xB0;
pub const SIGNATURE: u8 = 0x12;

#[derive(Debug, PartialEq, Eq, Clone, BoltStruct)]
pub struct Commit;

impl Commit {
    pub fn new() -> Commit {
        Commit {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn should_serialize_commit() {
        let commit = Commit::new();

        let bytes: Bytes = commit.try_into().unwrap();

        assert_eq!(bytes, Bytes::from_static(&[MARKER, SIGNATURE,]));
    }
}