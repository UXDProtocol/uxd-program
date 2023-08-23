const web3 = require('@solana/web3.js');
const bs58 = require('bs58');

exports.payer = web3.Keypair.fromSecretKey(
  bs58.decode(
    '5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG'
  )
);

exports.collateralMint = web3.Keypair.fromSecretKey(
  bs58.decode(
    '7MbiiCavjCmn9Hs1o4eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG'
  )
);
