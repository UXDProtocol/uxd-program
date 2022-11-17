// Unit tests
#[cfg(test)]
mod test_credix_lp_depository {
    use crate::state::credix_lp_depository::CREDIX_LP_DEPOSITORY_SPACE;
    use anchor_lang::prelude::Pubkey;
    use anchor_lang::Id;
    use anchor_lang::Result;
    use std::str::FromStr;

    #[test]
    fn test_credix_lp_depository_space() -> Result<()> {
        assert_eq!(
            credix_client::program::Credix::id(),
            Pubkey::from_str("CRdXwuY984Au227VnMJ2qvT7gPd83HwARYXcbHfseFKC").unwrap()
        );
        assert_eq!(CREDIX_LP_DEPOSITORY_SPACE, 1183);
        Ok(())
    }
}
