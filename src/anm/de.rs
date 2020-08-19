use super::shared::Animation;
use serde::de;
use serde_raw::from_raw_seed;
use std::io::Read;

pub fn animation_from_read<Source: Read>(
	source: &mut Source,
) -> Result<Animation, de::value::Error> {
	from_raw_seed(source, Animation::seed())
}
