use serde::de;
use serde_backtrace::serde_backtrace;
use serde_titan_quest::arc;
use std::{fs::File, io::BufReader, path::Path};
use test_case::test_case;

fn read_arc(path: impl AsRef<Path>) {
	let _: () = serde_backtrace(
		arc::from_read_seek_seed(
			&mut BufReader::new(File::open(&path).unwrap()),
			serde_backtrace::Seed(Seed(path)),
		)
		.unwrap(),
	);
}

#[test_case("Creatures.arc")]
#[test_case("Effects.arc")]
#[test_case("Fonts.arc")]
#[test_case("InGameUI.arc")]
#[test_case("Items.arc")]
#[test_case("Levels.arc")]
#[test_case("Lights.arc")]
#[test_case("Menu.arc")]
#[test_case("OutGameElements.arc")]
#[test_case("Particles.arc")]
#[test_case("Quests.arc")]
#[test_case("SceneryBabylon.arc")]
#[test_case("SceneryEgypt.arc")]
#[test_case("SceneryGreece.arc")]
#[test_case("SceneryOlympus.arc")]
#[test_case("SceneryOrient.arc")]
#[test_case("Shaders.arc")]
#[test_case("System.arc")]
#[test_case("TerrainTextures.arc")]
#[test_case("UI.arc")]
#[test_case("Underground.arc")]
#[test_case("../Audio/Dialog.arc")]
#[test_case("../Audio/Music.arc")]
#[test_case("../Audio/Sounds.arc")]
#[ignore = "requires Titan Quest to be installed"]
fn legacy(path: impl AsRef<Path>) {
	read_arc(
		Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\Titan Quest\Resources")
			.join(path),
	);
}

#[test]
fn dbg() {
	read_arc(r"C:\Program Files (x86)\Steam\steamapps\common\Titan Quest\Resources\Creatures.arc")
}

#[test_case("Creatures.arc")]
#[test_case("Effects.arc")]
#[test_case("Fonts.arc")]
#[test_case("InGameUI.arc")]
#[test_case("Items.arc")]
#[test_case("Levels.arc")]
#[test_case("Lights.arc")]
#[test_case("Menu.arc")]
#[test_case("Music.arc")]
// etc
#[ignore = "requires Titan Quest Anniversary Edition to be installed"]
fn anniversary_edition(path: impl AsRef<Path>) {
	read_arc(
		Path::new(
			r"C:\Program Files (x86)\Steam\steamapps\common\Titan Quest Anniversary Edition\Resources\",
		)
		.join(path),
	);
}

struct Seed<P>(P);
impl<'de, P> de::DeserializeSeed<'de> for Seed<P> {
	type Value = ();
	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_map(Visitor(self.0))
	}
}

struct Visitor<P>(P);
impl<'de, P> de::Visitor<'de> for Visitor<P> {
	type Value = ();
	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "ARC map")
	}

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: de::MapAccess<'de>,
	{
		while let Some(k) = map.next_key()? {
			let k: String = k;
			let data: Vec<u8> = map.next_value()?;
			if data.is_empty() {
				println!("{:?}", (data.len(), k));
			}
		}

		Ok(())
	}
}
