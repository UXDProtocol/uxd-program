use solana_program_test::processor;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;

pub async fn create_program_test_context() -> ProgramTestContext {
    // Program test struct will need to be aware of all its programs it will use
    let mut program_test = ProgramTest::default();

    // Deploy the uxd program from compiled artifact
    program_test.add_program("uxd", uxd::id(), processor!(uxd::entry));

    // For some reason we need to set this flag to true in order for the binaries files to be loaded as programs
    program_test.prefer_bpf(true);

    // Deploy the mercurial program using a downloaded mercurial compiled binary
    program_test.add_program(
        "tests/integration_tests/api/program_mercurial/binaries/executable-devnet",
        mercurial_vault::id(),
        None,
    );

    // Deploy the credix program using a downloaded credix compiled binary
    program_test.add_program(
        "tests/integration_tests/api/program_credix/binaries/executable-devnet-private",
        credix_client::id(),
        None,
    );

    // Done, generate the ProgramTestContext
    program_test.start_with_context().await
}
