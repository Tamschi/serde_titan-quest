use log::{error, info};
use serde_titan_quest::anm::de::animation_from_read;
use std::{
	fs::File,
	io::{BufReader, Seek as _, SeekFrom},
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
	info!("{:#?}", animation);
}

#[test]
#[ignore = r"Requires an animation file at C:\modding\tigerman_walk.anm."]
fn animation() {
	stderrlog::new().verbosity(3).init().ok();
	read_animation(r"C:\modding\tigerman_walk.anm")
}
