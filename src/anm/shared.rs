use {
    fn_t::Function,
    raw_seeders::{Literal, LittleEndian, Seq, Tuple, TupleN, IEEE754},
    serde_seeded::{seed, seeded, FunctionDeSeeder, FunctionSerSeeder},
};

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

    #[seeded(SeqN(bones, _<0>(self.frames)))]
    bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frames: u32)]
pub struct BoneAnimation {
    #[seeded(CP1252(LengthPrefixed::with::<u32, _>(LittleEndian)))]
    name: String,

    #[seeded_de(TupleN(frames as usize, FunctionDeSeeder(Frame::seed as fn(_) -> _)))]
    #[seeded_ser(TupleN(*frames as usize, FunctionSerSeeder(Frame::seeded as fn(_) -> _)))]
    frames: Vec<Frame>,

    #[seeded_de]
    #[seeded_ser(FunctionSerSeeder(Frame::seeded as fn(_) -> _))]
    frame: Frame,
}

#[derive(Debug, seed, seeded)]
pub struct Frame {
    #[seeded(Tuple::of(IEEE754(LittleEndian)))]
    offset: [f32; 3],

    #[seeded]
    q_1: Quaternion,

    #[seeded(Tuple::of(IEEE754(LittleEndian)))]
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
