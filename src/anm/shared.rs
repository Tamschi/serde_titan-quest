use raw_seeders::{literal, LittleEndian};
use serde_seeded::{seed, seeded};

#[derive(Debug, seed, seeded)]
pub struct Animation {
    #[seeded(literal(b"ANM\x02"))]
    magic: (),

    #[seeded(LittleEndian)]
    bones: u32,

    #[seeded(LittleEndian)]
    frames: u32,

    #[seeded(LittleEndian)]
    fps: u32,

    #[seeded(count(bones, BoneAnimation::seed(frames)))]
    bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frames: u32)]
pub struct BoneAnimation {
    #[seeded(cp1252(length_prefixed::<u32, _>(LittleEndian)))]
    name: String,

    #[seeded_de(flat_count(frames, Frame::seed))]
    #[seeded_ser(flat(Frame::seeded))]
    frames: Vec<Frame>,
}

#[derive(Debug, seed, seeded)]
pub struct Frame {
    #[seeded(flat(IEEE754))]
    data: [f32; 14],
}
