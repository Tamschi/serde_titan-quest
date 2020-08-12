use super::de;
use serde::{de::DeserializeSeed, Deserialize, Deserializer};
use std::{
    io::{Read, Seek},
    marker::PhantomData,
};

pub fn from_read_seek<'de, Source: Read + Seek, T: Deserialize<'de>>(
    data: &'de mut Source,
) -> Result<T, <de::Deserializer<Source> as Deserializer>::Error> {
    from_read_seek_seed(data, PhantomData)
}

pub fn from_read_seek_seed<'de, Source: Read + Seek, Seed: DeserializeSeed<'de>>(
    data: &'de mut Source,
    seed: Seed,
) -> Result<Seed::Value, <de::Deserializer<Source> as serde::de::Deserializer>::Error> {
    seed.deserialize(de::Deserializer(data))
}
