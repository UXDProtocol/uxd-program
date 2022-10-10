# Maple Solana contract reverse engineering

## Common Naming

baseMint: USDC token mint account (locker mint)
sharesMint: Pool specific token mint account

*_locker: Token account with "USDC" in there
*_shares: Token account with "Shares" in there

lender: Special syrup account, PDA derived a pool+owner (contains whitelisting status)

## Instructions

lenderInitialize
 - payer
 - owner
 - pool
 - sharesMint
 - lender
 - lockedShares
 - lenderShares

lenderDeposit
 - lender
 - lenderUser
 - pool
 - globals
 - poolLocker
 - sharesMint
 - lockedShares
 - lenderShares
 - lenderLocker

lenderUnlockDeposit
 - lender
 - lenderUser
 - pool
 - globals
 - lockedShares
 - lenderShares

withdrawalRequestInitialize
 - lender
 - lenderOwner
 - pool
 - globals
 - sharesMint
 - lenderShareAccount
 - withdrawalRequest
 - withdrawalRequestLocker

widthdrawlRequestExecute
 - widthdrawalRequest
 - lenderOwner
 - lender
 - lenderShareAccount
 - pool
 - globals
 - poolLocker
 - sharesMint
 - withdrawalRequestLocker
 - lenderLocker

## Structs

"Pool": 
 - globals (account "syrup::Globals", hardcoded struct)
 - delegate
 - pendingDelegate
 - delegateClaimable
 - config
 - baseMint (account "token::Mint", USDC)
 - locker (account "token::TokenAccount", USDC mint, owned by pool?)
 - totalValue
 - sharesMint
 - sharesOutstanding
 - nonce
 - poolBump
 - lockerBump
 - sharesMintBump
 - delegateClaimableBump

"Lender":
 - pool
 - owner
 - allowlisted
 - depositTs
 - lockedSharesAmount
 - lockedShares
 - lenderShares
 - lenderBump
 - lockedSharesBump

"WidthdrawalRequest":
 - pool
 - lender
 - createTs ?
 - shares
 - locker
 - nonce
 - bump
 - lockerBump

