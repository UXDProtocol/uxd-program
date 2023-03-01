// Unit tests
#[cfg(test)]
mod test_identity_depository {
    use anchor_lang::Result;
    use std::mem::size_of;
    use uxd::state::identity_depository::IDENTITY_DEPOSITORY_SPACE;

    #[test]
    fn test_identity_depository_space() -> Result<()> {
        assert_eq!(IDENTITY_DEPOSITORY_SPACE, 640);
        assert_eq!(
            size_of::<uxd::state::identity_depository::IdentityDepository>(),
            IDENTITY_DEPOSITORY_SPACE - 8
        );
        Ok(())
    }
}
