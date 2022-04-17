use crate::*;

// 10000 equiv. to 100%
pub const LIQUIDITY_RATIO_BASIS: u16 = 10000;

pub const TARGET_LIQUIDITY_RATIO_MAX: u16 = 10000;

const MSOL_CONFIG_PADDING: usize = 64;

#[account]
#[derive(Default)]
pub struct MSolConfig {
    pub bump: u8,
    // The Depository that this config account dedicated to
    pub depository: Pubkey,
    // The Controller that own this config account
    pub controller: Pubkey,
    // Whether allowing the SOL <-> mSOL conversion
    pub enabled: bool,
    // A constant value of the proportion of SOL we wanna keep from the total amount of SOL + mSOL in terms of their value.
    // In LIQUIDITY_RATIO_BASIS
    pub target_liquidity_ratio: u16,
    // Reserved space meant to be used for future addition of state variables
    pub _reserved: MSolConfigPadding,
}

#[derive(Clone)]
pub struct MSolConfigPadding([u8; MSOL_CONFIG_PADDING]);

impl AnchorSerialize for MSolConfigPadding {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl AnchorDeserialize for MSolConfigPadding {
    fn deserialize(_: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self([0u8; MSOL_CONFIG_PADDING]))
    }
}

impl Default for MSolConfigPadding {
    fn default() -> Self {
        MSolConfigPadding([0u8; MSOL_CONFIG_PADDING])
    }
}
