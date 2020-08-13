use cast::i8;
use encoding::{all::WINDOWS_1252, DecoderTrap, Encoding as _};
use enum_ordinalize::Ordinalize;
use raw_seeders::{literal, LittleEndian};
use serde::{de, ser};
use serde_seeded::{seed, seeded};
use std::{borrow::Cow, convert::TryInto, fmt::Display, marker::PhantomData};

#[derive(Debug, seed, seeded)]
pub struct Header {
    #[seeded(literal(b"ARC\0"))]
    magic: (),

    #[seeded(LittleEndian)]
    version: u32,

    #[seeded(LittleEndian)]
    pub asset_count: u32,

    #[seeded(LittleEndian)]
    pub part_count: u32,

    unknown: [u8; 8],

    #[seeded(LittleEndian)]
    pub part_info_offset: u32,
}
impl Header {
    pub fn string_offset(&self) -> u32 {
        self.part_info_offset + self.part_count * PartInfo::SIZE as u32
    }

    pub fn asset_info_offset_from_end(&self) -> u32 {
        self.asset_count * AssetInfo::SIZE as u32
    }
}

#[derive(seed, seeded)]
pub struct PartInfo {
    #[seeded(LittleEndian)]
    pub offset: u32,

    #[seeded(LittleEndian)]
    pub compressed_length: u32,

    #[seeded(LittleEndian)]
    pub data_length: u32,
}
impl PartInfo {
    pub const SIZE: usize = 12;
}

#[derive(seed, seeded)]
pub struct AssetInfo {
    #[seeded]
    pub storage: Storage, //u32

    #[seeded(LittleEndian)]
    pub offset: u32,

    #[seeded(LittleEndian)]
    pub compressed_length: u32,

    #[seeded(LittleEndian)]
    pub asset_length: u32,

    unknown: [u8; 12],

    #[seeded(LittleEndian)]
    pub part_count: u32,

    #[seeded(LittleEndian)]
    pub first_part: u32,

    #[seeded(LittleEndian)]
    pub name_length: u32,

    #[seeded(LittleEndian)]
    pub name_offset: u32,
}
impl AssetInfo {
    pub const SIZE: usize = 44;
}

#[derive(Clone, Copy, Ordinalize)]
pub enum Storage {
    Uncompressed = 1,
    Compressed = 3,
}
impl Storage {
    fn seed<'de>() -> impl de::DeserializeSeed<'de, Value = Self> {
        struct Seed;
        impl<'de> de::DeserializeSeed<'de> for Seed {
            type Value = Storage;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u32::from_le_bytes(PhantomData.deserialize(deserializer)?);
                let value = i8(value).map_err(de::Error::custom)?;
                let value = Storage::from_ordinal(value).ok_or_else(|| {
                    de::Error::invalid_value(
                        de::Unexpected::Signed(value as i64),
                        &"a Storage value",
                    )
                })?;
                Ok(value)
            }
        }

        Seed
    }
    fn seeded(&self) -> impl ser::Serialize {
        (self.ordinal() as u32).to_le_bytes()
    }
}

pub fn cp1252<'a>(data: Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>> {
    Ok(WINDOWS_1252
        .decode(data.as_ref(), DecoderTrap::Strict)
        .map_err(|e| Box::new(e) as _)?
        .into())
}
