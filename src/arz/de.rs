use serde::{
    de::{self, Error, IntoDeserializer as _},
    forward_to_deserialize_any,
};
use std::io::{Read, Seek};

pub struct Deserializer<'de, Source>(pub &'de mut Source);
impl<'de, Source: Read + Seek> de::Deserializer<'de> for Deserializer<'de, Source> {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
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
