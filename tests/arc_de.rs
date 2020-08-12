use std::{fs::File, path::Path};
use {serde_object::Object, serde_titan_quest::arc, test_case::test_case};

fn read_arc(path: impl AsRef<Path>) {
    let _: Object = arc::from_read_seek(&mut File::open(path).unwrap()).unwrap();
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
    read_arc(Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\Titan Quest\Resources").join(path));
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
    read_arc(Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\Titan Quest Anniversary Edition\Resources\").join(path));
}
