use anchor_lang::prelude::Pubkey;

pub fn find_controller_address() -> Pubkey {
    let (controller, _) =
        Pubkey::find_program_address(&[uxd::CONTROLLER_NAMESPACE.as_ref()], &uxd::id());
    assert_eq!(
        "3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW",
        controller.to_string()
    );
    return controller;
}
