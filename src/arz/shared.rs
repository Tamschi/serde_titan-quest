use raw_seeders::{literal, LittleEndian};
use serde_seeded::{seed, seeded};

#[derive(Debug, seed, seeded)]
pub struct Header {
    #[seeded(literal(b"ARZ\0"))]
    magic: (),

    #[seeded(LittleEndian)]
    record_table_offset: u32,

    #[seeded(LittleEndian)]
    record_table_size: u32,

    #[seeded(LittleEndian)]
    record_table_lenth: u32,

    #[seeded(LittleEndian)]
    string_table_offset: u32,

    #[seeded(LittleEndian)]
    string_table_size: u32,
}

#[derive(Debug, seed, seeded)]
pub struct Footer {
    #[seeded(LittleEndian)]
    checksum: u32,

    #[seeded(LittleEndian)]
    checksum_string_table: u32,

    #[seeded(LittleEndian)]
    checksum_records: u32,

    #[seeded(LittleEndian)]
    checksum_records_table: u32,
}
impl Footer {
    pub const SIZE: usize = 16;
}
