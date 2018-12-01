#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const EMAC0_BASE: u32 = 0x400ec000;
pub const SYSCTL_PERIPH_EMAC0: u32 = 0xf0009c00;
pub const SYSCTL_PERIPH_EPHY0: u32 = 0xf0003000;
pub const EMAC_CONFIG_USE_MACADDR0: u32 = 0x0;
pub const EMAC_PHY_TYPE_INTERNAL: u32 = 0x0;
pub const EMAC_CONFIG_IF_GAP_40BITS: u32 = 0x7 << 17;
pub const EMAC_CONFIG_3BYTE_PREAMBLE: u32 = 0x00000002;
pub const EMAC_CONFIG_BO_LIMIT_2: u32 = 0x3 << 5;
pub const EMAC_CONFIG_SA_INSERT: u32 = 0x20000000;
pub const EMAC_CONFIG_FULL_DUPLEX: u32 = 0x00000800;
pub const EMAC_MODE_TX_THRESHOLD_16_BYTES: u32 = 7 << 14;
pub const EMAC_MODE_RX_THRESHOLD_64_BYTES: u32 = 0 << 3;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
