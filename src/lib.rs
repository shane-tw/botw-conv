mod consts;

use std::path::Path;
use std::fmt;
use std::io::{self, Read, Write, Seek, SeekFrom, Error, ErrorKind};
use std::ffi::OsStr;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum SavePlatform {
	WiiU,
	Switch
}

impl fmt::Display for SavePlatform {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			SavePlatform::WiiU => write!(f, "WiiU"),
			SavePlatform::Switch => write!(f, "Switch"),
		}
	}
}

impl PartialEq for SavePlatform {
	fn eq(&self, other: &SavePlatform) -> bool {
		*self as u8 == *other as u8
	}
}

pub fn get_save_platform<R: Read + Seek>(option_sav: &mut R) -> io::Result<SavePlatform> {
	let mut first_byte = [0; 1];

	option_sav.seek(SeekFrom::Start(0))?;
	option_sav.read(&mut first_byte)?;

	Ok(match first_byte.contains(&0) {
		true => SavePlatform::WiiU,
		_ => SavePlatform::Switch,
	})
}

pub fn get_save_version<R: Read + Seek>(game_data_sav: &mut R, save_platform: &SavePlatform) -> io::Result<String> {
	let file_size = game_data_sav.seek(SeekFrom::End(0))?;
	game_data_sav.seek(SeekFrom::Start(0))?;

	let mut header2 = [0; 2];
	let mut header3 = [0; 3];

	if *save_platform == SavePlatform::WiiU {
		let mut searching = true;
		while searching {
			let mut byte = [0; 1];
			game_data_sav.read(&mut byte)?;
			searching = byte.contains(&0);
		}
		game_data_sav.seek(SeekFrom::Current(-1))?;
	}

	game_data_sav.read(&mut header2)?;
	game_data_sav.seek(SeekFrom::Start(0))?;
	game_data_sav.read(&mut header3)?;

	if *save_platform == SavePlatform::Switch {
		header2.reverse();
		header3.reverse();
	}

	for i in 0 .. consts::HEADERS_2.len() {
		if header2 == *consts::HEADERS_2[i] && file_size == consts::FILE_SIZES[i] {
			return Ok(format!("{}", consts::VERSIONS[i]));
		}
		// check for mods, filesizes vary
		if header2 == *consts::HEADERS_2[i] && (file_size >= 896976 && file_size <= 1500000) {
			return Ok(format!("{} mod", consts::VERSIONS[i]));
		}
	}

	// TODO: Look into a way of reducing duplication here
	for i in 0 .. consts::HEADERS_3.len() {
		let sizes_idx = i + consts::HEADERS_2.len();
		if header3 == *consts::HEADERS_3[i] && file_size == consts::FILE_SIZES[sizes_idx] {
			return Ok(format!("{}", consts::VERSIONS[i]));
		}
		// check for mods, filesizes vary
		if header3 == *consts::HEADERS_3[i] && (file_size >= 896976 && file_size <= 1500000) {
			return Ok(format!("{} mod", consts::VERSIONS[i]));
		}
	}

	Err(Error::new(ErrorKind::Other, "Version unknown"))
}

pub fn convert_save<RW: Read + Write + Seek>(sav: &mut RW, sav_path: &Path) -> io::Result<()> {
	let file_name = sav_path.file_name()
		.and_then(OsStr::to_str)
		.and_then(|s| Some(s.to_string()))
		.unwrap_or(".".to_string());

	if sav_path.extension() != Some(OsStr::new("sav")) {
		return Ok(());
	}

	let file_size = sav.seek(SeekFrom::End(0))?;
	sav.seek(SeekFrom::Start(0))?;

	let mut position: u64;
	let mut h: u64 = 0;

	while h < file_size / 4 {
		if file_name.starts_with("trackblock") && h == 0 {
			position = 4;
			let mut header = [0; 2];

			sav.seek(SeekFrom::Start(position))?;
			sav.read(&mut header)?;

			header.reverse();

			sav.seek(SeekFrom::Start(position))?;
			sav.write(&header)?;
			h = 2;
		}

		position = h * 4;
		let mut bytes = [0; 4];

		sav.seek(SeekFrom::Start(position))?;
		sav.read(&mut bytes)?;

		let hash_match = consts::HASHES_4.contains(&&bytes);
		let byte_str = String::from_utf8_lossy(&bytes);

		if hash_match || !consts::ITEMS.iter().any(|item| byte_str.contains(item)) {
			bytes.reverse();

			sav.seek(SeekFrom::Start(position))?;
			sav.write(&bytes)?;

			if hash_match {
				h += 1;
			}
		} else {
			h += 1;
			for i in 0 .. 16 {
				position = (h + (i * 2)) * 4;
				let mut endian_hash = [0; 4];

				sav.seek(SeekFrom::Start(position))?;
				sav.read(&mut endian_hash)?;

				endian_hash.reverse();

				sav.seek(SeekFrom::Start(position))?;
				sav.write(&mut endian_hash)?;
			}
			h += 30;
		}
		h += 1;
	}
	Ok(())
}
