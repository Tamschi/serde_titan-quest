use log::error;
use serde_titan_quest::anm::de::animation_from_read;
use std::{
	fs::File,
	io::Seek as _,
	io::{BufReader, SeekFrom},
	path::Path,
};

fn read_animation(path: impl AsRef<Path>) {
	let mut file = BufReader::new(File::open(&path).unwrap());
	let animation = animation_from_read(&mut file);
	if animation.is_err() {
		error!(
			"Failed to read animation: {:?}; File position: {:x}",
			animation,
			file.seek(SeekFrom::Current(0)).unwrap()
		);
	}
	// dbg!(animation).unwrap();
}

#[test]
#[ignore = r"Requires an animation file at C:\modding\tigerman_walk.anm."]
fn animation() {
	stderrlog::new().verbosity(3).init().unwrap();
	read_animation(r"C:\modding\tigerman_walk.anm")
}
