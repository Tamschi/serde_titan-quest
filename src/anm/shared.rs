use raw_seeders::{Literal, LittleEndian, IEEE754, Tuple};
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

    #[seeded_de(SeqN(bones, *_(frames)))]
    #[seeded_ser(Seq(*_))]
    bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frames: u32)]
pub struct BoneAnimation {
    #[seeded(cp1252(length_prefixed::<u32, _>(LittleEndian)))]
    name: String,

    #[seeded_de(SeqN(frames, *_))]
    #[seeded_ser(Seq(*_))]
    frames: Vec<Frame>,
}

#[derive(Debug, seed, seeded)]
pub struct Frame {
    #[seeded(Tuple(IEEE754(LittleEndian)))]
    offset: [f32; 3],

    #[seeded]
    q_1: Quaternion,

    #[seeded(Tuple(IEEE754(LittleEndian)))]
    scale: [f32; 3],

    #[seeded]
    q_2: Quaternion,
}

#[derive(Debug, seed, seeded)]
pub struct Quaternion {
    #[seeded(IEEE754(LittleEndian))]
    i: f32,

    #[seeded(IEEE754(LittleEndian))]
    j: f32,

    #[seeded(IEEE754(LittleEndian))]
    k: f32,

    #[seeded(IEEE754(LittleEndian))]
    w: f32,
}
