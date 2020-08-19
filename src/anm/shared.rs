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

	#[seeded_de(TupleN(bones as usize, FnDeSeeder(|| BoneAnimation::seed(frames as usize))))]
	#[seeded_ser(TupleN(*bones as usize, FnSerSeeder::new(|b| Box::new(BoneAnimation::seeded(b, *frames as usize)))))]
	bone_animations: Vec<BoneAnimation>,
}

#[derive(Debug, seed, seeded)]
#[seed_args(frame_count: usize)]
pub struct BoneAnimation {
	#[seeded(Windows1252(LengthPrefixed(TryAsU32(LittleEndian), SerdeLike)))]
	name: String,

	#[seeded_de(TupleN(frame_count, FnDeSeeder(Frame::seed)))]
	//TODO: Pass by value/consume seeder instead when serializing to avoid reference discrepancy?
	#[seeded_ser(TupleN(*frame_count, FnSerSeeder::new(|f| Box::new(Frame::seeded(f)))))]
	frames: Vec<Frame>,
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
