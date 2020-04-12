pub const MAX_SCRIPT_BLOCK: usize = 50;

pub const BTCHIP_CLA: u8 = 0xe0;
pub const BTCHIP_JC_EXT_CLA: u8 = 0xf0;

pub const BTCHIP_INS_SET_ALTERNATE_COIN_VERSION: u8 = 0x14;
pub const BTCHIP_INS_SETUP: u8 = 0x20;
pub const BTCHIP_INS_VERIFY_PIN: u8 = 0x22;
pub const BTCHIP_INS_GET_OPERATION_MODE: u8 = 0x24;
pub const BTCHIP_INS_SET_OPERATION_MODE: u8 = 0x26;
pub const BTCHIP_INS_SET_KEYMAP: u8 = 0x28;
pub const BTCHIP_INS_SET_COMM_PROTOCOL: u8 = 0x2a;
pub const BTCHIP_INS_GET_WALLET_PUBLIC_KEY: u8 = 0x40;
pub const BTCHIP_INS_GET_TRUSTED_INPUT: u8 = 0x42;
pub const BTCHIP_INS_HASH_INPUT_START: u8 = 0x44;
pub const BTCHIP_INS_HASH_INPUT_FINALIZE: u8 = 0x46;
pub const BTCHIP_INS_HASH_SIGN: u8 = 0x48;
pub const BTCHIP_INS_HASH_INPUT_FINALIZE_FULL: u8 = 0x4a;
pub const BTCHIP_INS_GET_INTERNAL_CHAIN_INDEX: u8 = 0x4c;
pub const BTCHIP_INS_SIGN_MESSAGE: u8 = 0x4e;
pub const BTCHIP_INS_GET_TRANSACTION_LIMIT: u8 = 0xa0;
pub const BTCHIP_INS_SET_TRANSACTION_LIMIT: u8 = 0xa2;
pub const BTCHIP_INS_IMPORT_PRIVATE_KEY: u8 = 0xb0;
pub const BTCHIP_INS_GET_PUBLIC_KEY: u8 = 0xb2;
pub const BTCHIP_INS_DERIVE_BIP32_KEY: u8 = 0xb4;
pub const BTCHIP_INS_SIGNVERIFY_IMMEDIATE: u8 = 0xb6;
pub const BTCHIP_INS_GET_RANDOM: u8 = 0xc0;
pub const BTCHIP_INS_GET_ATTESTATION: u8 = 0xc2;
pub const BTCHIP_INS_GET_FIRMWARE_VERSION: u8 = 0xc4;
pub const BTCHIP_INS_COMPOSE_MOFN_ADDRESS: u8 = 0xc6;
pub const BTCHIP_INS_GET_POS_SEED: u8 = 0xca;

pub const BTCHIP_INS_EXT_GET_HALF_PUBLIC_KEY: u8 = 0x20;
pub const BTCHIP_INS_EXT_CACHE_PUT_PUBLIC_KEY: u8 = 0x22;
pub const BTCHIP_INS_EXT_CACHE_HAS_PUBLIC_KEY: u8 = 0x24;
pub const BTCHIP_INS_EXT_CACHE_GET_FEATURES: u8 = 0x26;

pub const OPERATION_MODE_WALLET: u8 = 0x01;
pub const OPERATION_MODE_RELAXED_WALLET: u8 = 0x02;
pub const OPERATION_MODE_SERVER: u8 = 0x04;
pub const OPERATION_MODE_DEVELOPER: u8 = 0x08;

pub const FEATURE_UNCOMPRESSED_KEYS: u8 = 0x01;
pub const FEATURE_RFC6979: u8 = 0x02;
pub const FEATURE_FREE_SIGHASHTYPE: u8 = 0x04;
pub const FEATURE_NO_2FA_P2SH: u8 = 0x08;
