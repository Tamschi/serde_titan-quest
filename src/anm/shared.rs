use raw_seeders::{Literal, LittleEndian};
use serde_seeded::{seed, seeded};

#[derive(Debug, seed, seeded)]
pub struct Animation {
    #[seeded(Literal(b"ANM\x02"))]
    magic: (),

    #[seeded(LittleEndian)]
    bones: u32,

    #[seeded(LittleEndian)]
    frames: u32,

    #[seeded(LittleEndian)]
    fps: u32,

    #[seeded_de(flat_count(bones, *_(frames)))]
    #[seeded_ser(flat_ser(*_))]
    bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frames: u32)]
pub struct BoneAnimation {
    #[seeded(cp1252(length_prefixed::<u32, _>(LittleEndian)))]
    name: String,

    #[seeded_de(flat_count(frames, *_))]
    #[seeded_ser(flat_ser(*_))]
    frames: Vec<Frame>,
}

#[derive(Debug, seed, seeded)]
pub struct Frame {
    #[seeded(flat(LittleEndian))]
    offset: [f32; 3],

    #[seeded]
    q_1: Quaternion,

    #[seeded(flat(LittleEndian))]
    scale: [f32; 3],

    #[seeded]
    q_2: Quaternion,
}

#[derive(Debug, seed, seeded)]
pub struct Quaternion {
    #[seeded(LittleEndian)]
    i: f32,

    #[seeded(LittleEndian)]
    j: f32,

    #[seeded(LittleEndian)]
    k: f32,

    #[seeded(LittleEndian)]
    w: f32,
}
