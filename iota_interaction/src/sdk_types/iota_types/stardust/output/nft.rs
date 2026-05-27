// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_types::{Identifier, StructTag};
use crate::types::base_types::IotaAddress;

pub const NFT_MODULE_NAME: Identifier = Identifier::from_static("nft");
pub const NFT_OUTPUT_MODULE_NAME: Identifier = Identifier::from_static("nft_output");
pub const NFT_OUTPUT_STRUCT_NAME: Identifier = Identifier::from_static("NftOutput");
pub const NFT_STRUCT_NAME: Identifier = Identifier::from_static("Nft");
pub const NFT_DYNAMIC_OBJECT_FIELD_KEY: &[u8] = b"nft";
pub const NFT_DYNAMIC_OBJECT_FIELD_KEY_TYPE: &str = "vector<u8>";

pub struct Nft {}

impl Nft {
    /// Returns the struct tag that represents the fully qualified path of an
    /// [`Nft`] in its move package.
    pub fn tag() -> StructTag {
        StructTag::new(
            IotaAddress::STARDUST,
            NFT_MODULE_NAME,
            NFT_STRUCT_NAME,
            Vec::new(),
        )
    }
}
