const web3 = require('@solana/web3.js');
const token = require('@solana/spl-token');
const utils = require('./utils.js');
const keys = require('./keys.js');

const connection = new web3.Connection('https://api.devnet.solana.com', {
  commitment: 'confirmed',
});

async function main() {
  const mint = await token.createMint(
    connection,
    keys.payer,
    keys.collateralMint.publicKey,
    keys.collateralMint.publicKey,
    6
  );
  console.log('mint', mint);
}

main();
