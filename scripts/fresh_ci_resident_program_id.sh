#!/bin/sh

# The CI uses a program that persist until this script is called.
# This script replace all occurence of that program in the code to replace it with a new one, 
# then give the upgrade authority to the specific keypair used by the CI for upgrading that program.

CI_RESIDENT_KEYPAIR_PATH="./target/deploy/ci-resident-keypair.json"
CI_RESIDENT_UPGRADE_AUTHORITY_KEYPAIR_PATH="./target/deploy/ci-resident-upgrade-authority.json"
DEFAULT_DEPLOYMENT_KEYPAIR_PATH="./target/deploy/uxd-keypair.json"

# Get current resident keypair's Pubkey
OLD_PUBKEY=`solana-keygen pubkey $CI_RESIDENT_KEYPAIR_PATH`
echo $OLD_PUBKEY

# Create new keypair and do the replacement in code
./scripts/reset_program_id.sh

# build new program
anchor build

# deploy new program
anchor deploy

# Get the new CI Resident program keypair's Pubkey
NEW_CI_RESIDENT_PROGRAM_PUBKEY=`solana-keygen pubkey $DEFAULT_DEPLOYMENT_KEYPAIR_PATH`
echo $NEW_CI_RESIDENT_PROGRAM_PUBKEY

# copy the secret key of the newly deployed program into the ci-resident-keypair.sh file
cat $DEFAULT_DEPLOYMENT_KEYPAIR_PATH > $CI_RESIDENT_KEYPAIR_PATH

# give the ci upgrade auth key the right over this new program, so that it can upgrade it
CI_UPGRADE_AUTHORITY_RESIDENT_PUBKEY=`solana-keygen pubkey $CI_RESIDENT_UPGRADE_AUTHORITY_KEYPAIR_PATH`
solana program set-upgrade-authority $NEW_CI_RESIDENT_PROGRAM_PUBKEY --new-upgrade-authority $CI_UPGRADE_AUTHORITY_RESIDENT_PUBKEY

# Replace occurance in doc and files around the workspace
sed -i.bak "s/$OLD_PUBKEY/$NEW_CI_RESIDENT_PROGRAM_PUBKEY/g" ./scripts/swap_ci_resident_program_id.sh
sed -i.bak "s/$OLD_PUBKEY/$NEW_CI_RESIDENT_PROGRAM_PUBKEY/g" ./.github/workflows/ci-anchor-test.yml
sed -i.bak "s/$OLD_PUBKEY/$NEW_CI_RESIDENT_PROGRAM_PUBKEY/g" ./README.md