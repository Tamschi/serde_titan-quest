use super::shared::{cp1252, AssetInfo, Header, PartInfo};
use miniz_oxide::inflate::decompress_to_vec;
use serde::{
	de::{self, Error, IntoDeserializer as _},
	forward_to_deserialize_any,
};
use serde_raw::{
	from_raw_seed, from_raw_seed_at,
	mapping::{fixed_len, string},
};
use std::io::{Read, Seek, SeekFrom};

pub struct Deserializer<'de, Source: Read + Seek>(pub &'de mut Source);
impl<'de, Source: Read + Seek> de::Deserializer<'de> for Deserializer<'de, Source> {
	type Error = de::value::Error;
	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let start = self.0.seek(SeekFrom::Current(0)).map_err(Error::custom)?;
		let header: Header = from_raw_seed(self.0, Header::seed())?;

		visitor.visit_map(FilesAccess {
			source: self.0,
			start,
			header,
			i: 0,
		})
	}

	forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map struct enum identifier
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_unit()
	}
}

struct FilesAccess<'de, Source: Read + Seek> {
	source: &'de mut Source,
	start: u64,
	header: Header,
	i: u32,
}

impl<'de, Source: Read + Seek> de::MapAccess<'de> for FilesAccess<'de, Source> {
	type Error = de::value::Error;
	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: de::DeserializeSeed<'de>,
	{
		if self.i < self.header.asset_count {
			let asset_info: AssetInfo = from_raw_seed_at(
				self.source,
				AssetInfo::seed(),
				SeekFrom::End(
					-(self.header.asset_info_offset_from_end() as i64)
						+ self.i as i64 * AssetInfo::SIZE as i64,
				),
			)?;

			let name = from_raw_seed_at(
				self.source,
				string(fixed_len(asset_info.name_length as usize), cp1252),
				SeekFrom::Start(
					self.start + self.header.string_offset() as u64 + asset_info.name_offset as u64,
				),
			)?;

			let key = seed.deserialize(name.into_deserializer())?;
			self.i += 1;
			Ok(Some(key))
		} else {
			Ok(None)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: de::DeserializeSeed<'de>,
	{
		let asset_info: AssetInfo = from_raw_seed_at(
			self.source,
			AssetInfo::seed(),
			SeekFrom::End(
				-(self.header.asset_info_offset_from_end() as i64)
					+ (self.i - 1) as i64 * AssetInfo::SIZE as i64,
			),
		)?;

		let mut parts = vec![];
		self.source
			.seek(SeekFrom::Start(
				self.start
					+ self.header.part_info_offset as u64
					+ asset_info.first_part as u64 * PartInfo::SIZE as u64,
			))
			.map_err(de::Error::custom)?;
		for _ in 0..asset_info.part_count {
			let part: PartInfo = from_raw_seed(self.source, PartInfo::seed())?;
			parts.push(part)
		}

		let mut data: Vec<u8> = vec![];
		match asset_info.storage {
			super::shared::Storage::TODODeleted => {}
			super::shared::Storage::Uncompressed => {
				self.source
					.seek(SeekFrom::Start(self.start + asset_info.offset as u64))
					.map_err(de::Error::custom)?;
				data.resize_with(asset_info.asset_length as usize, Default::default);
				self.source
					.read_exact(&mut data)
					.map_err(de::Error::custom)?;
			}
			super::shared::Storage::Compressed => {
				for part in parts {
					let mut compressed: Vec<u8> =
						Vec::with_capacity(part.compressed_length as usize - 2);
					self.source
						.seek(SeekFrom::Start(
							self.start + part.offset as u64 + 2, /* "Skip zlib flags", TODO: Do this more nicely. */
						))
						.map_err(de::Error::custom)?;
					compressed.resize_with(part.compressed_length as usize, Default::default);
					self.source
						.read_exact(&mut compressed)
						.map_err(de::Error::custom)?;
					let mut decompressed = decompress_to_vec(&compressed)
						.map_err(|e| de::Error::custom(format_args!("{:?}", e)))?;
					data.append(&mut decompressed);
				}
			}
		}

		seed.deserialize(data.into_deserializer())
	}
}
