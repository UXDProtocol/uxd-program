// Unit tests
#[cfg(test)]
mod test_controller {
    use anchor_lang::Result;
    use std::mem::size_of;
    use uxd::state::controller::CONTROLLER_SPACE;

    #[test]
    fn test_controller_space() -> Result<()> {
        assert_eq!(CONTROLLER_SPACE, 885);
        assert_eq!(size_of::<uxd::state::Controller>(), CONTROLLER_SPACE - 8);
        Ok(())
    }
}
