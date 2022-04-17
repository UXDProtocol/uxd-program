# mSOL program design
Here is the high level program design for keeping part of the SOL collateral of UXD as mSOL and how to maintain a constant liquidity ratio. 

Reward harvesting would be expected to add in the second stage of the whole mSOL implementation. (probably by regularly drawing mSOL from the Dex Depository Account, TBC)
___
## changes for state

added a new `msol_config` account state.
```Zsh
pub struct MSolConfig {
    ...
    pub depository: Pubkey,
    pub controller: Pubkey,
    pub enabled: bool,
    pub target_liquidity_ratio: u16,
    ...
}
```
Authority of the controller must create a MSolConfig first for one of it's depository before running the mSOL/SOL conversion instructions. This state is intentionally decoupled with the depository. Several reason of this implementation, 
1. to avoid any prior execution of the permissionless call for the conversion ixs before specifying the target liquidity ratio
2. having a `MSolConfig` on depository state is wasting space for most cases, since it's only useful for depository with collateral mint as SOL
3. no modifying of existing depository related instructions
___
## changes for instructions
### permissionned ixns:
### `create_depository_msol_config`
To initialize `msol_config` account, specify the `target_liquidity_ratio` and setup it's relationship to the `controller` and `depository`, could only called by the controller's authority. For each `depository`, there could only hv one `msol_config` created. 

