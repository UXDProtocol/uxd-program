const web3 = require('@solana/web3.js');
const token = require('@solana/spl-token');
const utils = require('./utils.js');
const keys = require('./keys.js');

const connection = new web3.Connection('https://api.devnet.solana.com', {
  commitment: 'confirmed',
});

async function main() {
  const tutu1 = web3.Keypair.generate();
  const tutu2 = web3.Keypair.generate();

  console.log('tutu1', tutu1);
  console.log('tutu2', tutu2);

  console.log('keys.payer.publicKey', keys.payer.publicKey);
  console.log('keys.collateralMint.publicKey', keys.collateralMint.publicKey);

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
