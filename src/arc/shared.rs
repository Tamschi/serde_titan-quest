use encoding::{all::WINDOWS_1252, DecoderTrap, Encoding as _};
use serde::{Deserialize, Serialize};
use serde_seeded::{seed, seeded};
use std::{borrow::Cow, fmt::Display};

#[derive(seed, seeded)]
pub struct Header {
    magic: (),
    version: u32,
    pub file_count: u32,
    pub part_count: u32,
    unknown: [u8; 8],
    pub part_info_offset: u32,
    pub string_offset: u32,
    pub asset_info_offset: u32,
}

#[derive(seed, seeded)]
pub struct PartInfo {
    pub offset: u32,
    pub compressed_length: u32,
    pub data_length: u32,
}
impl PartInfo {
    pub const SIZE: usize = 12;
}

#[derive(seed, seeded)]
pub struct AssetInfo {
    pub storage: Storage, //u32
    pub offset: u32,
    pub compressed_length: u32,
    pub asset_length: u32,
    unknown: [u8; 12],
    pub part_count: u32,
    pub first_part: u32,
    pub name_length: u32,
    pub name_offset: u32,
}
impl AssetInfo {
    pub const SIZE: usize = 44;
}

#[derive(Serialize, Deserialize)]
pub enum Storage {}

pub fn cp1252<'a>(data: Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>> {
    Ok(WINDOWS_1252
        .decode(data.as_ref(), DecoderTrap::Strict)
        .map_err(|e| Box::new(e) as _)?
        .into())
}
