use raw_seeders::{
	LengthPrefixed, Literal, LittleEndian, SerdeLike, TryAsU32, Tuple, TupleN, Windows1252, IEEE754,
};
use serde_seeded::{seed, seeded, FnDeSeeder, FnSerSeeder};

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
	// #[seeded(SeqN(bones, _<0>(self.frames)))]
	// bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frame_count: u32)]
pub struct BoneAnimation {
	#[seeded(Windows1252(LengthPrefixed(TryAsU32(LittleEndian), SerdeLike)))]
	name: String,

	#[seeded(LengthPrefixed(TryAsU32(LittleEndian), SerdeLike))]
	length_prefixed_test: Vec<u8>,

	#[seeded_de(TupleN(frame_count as usize, FnDeSeeder(Frame::seed)))]
	//TODO: Pass by value/consume seeder instead when serializing to avoid pointer stuff?
	#[seeded_ser(TupleN(*frame_count as usize, FnSerSeeder::new(|f| Box::new(Frame::seeded(f)))))]
	frames: Vec<Frame>,

	#[seeded]
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
