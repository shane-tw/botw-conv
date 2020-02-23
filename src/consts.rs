pub const ITEMS: [&str; 60] = [
	"Item", "Weap", "Armo", "Fire", "Norm", "IceA", "Elec", "Bomb", "Anci", "Anim",
	"Obj_", "Game", "Dm_N", "Dm_A", "Dm_E", "Dm_P", "FldO", "Gano", "Gian", "Grea",
	"KeyS", "Kokk", "Liza", "Mann", "Mori", "Npc_", "OctO", "Octa", "Octa", "arro",
	"Pict", "PutR", "Rema", "Site", "TBox", "TwnO", "Prie", "Dye0", "Dye1", "Map",
	"Play", "Oasi", "Cele", "Wolf", "Gata", "Ston", "Kaka", "Soji", "Hyru", "Powe",
	"Lana", "Hate", "Akka", "Yash", "Dung", "BeeH", "Boar", "Boko", "Brig", "DgnO"
];

pub const HASHES_4: [&[u8; 4]; 44] = [
	b"\x7B\x74\xE1\x17", b"\x17\xE1\x74\x7B", b"\xD9\x13\xB7\x69", b"\x69\xB7\x13\xD9",
	b"\xB6\x66\xD2\x46", b"\x46\xD2\x66\xB6", b"\x02\x1A\x6F\xF2", b"\xF2\x6F\x1A\x02",
	b"\xFF\x74\x96\x0F", b"\x0F\x96\x74\xFF", b"\x89\x32\x28\x5F", b"\x5F\x28\x32\x89",
	b"\x3B\x0A\x28\x9B", b"\x9B\x28\x0A\x3B", b"\x2F\x95\x76\x8F", b"\x8F\x76\x95\x2F",
	b"\x9C\x6C\xFD\x3F", b"\x3F\xFD\x6C\x9C", b"\xBB\xAC\x41\x6B", b"\x6B\x41\xAC\xBB",
	b"\xCC\xAB\x71\xFD", b"\xFD\x71\xAB\xCC", b"\xCB\xC6\xB5\xE4", b"\xE4\xB5\xC6\xCB",
	b"\x2C\xAD\xB0\xE7", b"\xE7\xB0\xAD\x2C", b"\xA6\xEB\x3E\xF4", b"\xF4\x3E\xEB\xA6",
	b"\x21\xD4\xCF\xFA", b"\xFA\xCF\xD4\x21", b"\x22\xA5\x10\xD1", b"\xD1\x10\xA5\x22",
	b"\x98\xD1\x0D\x53", b"\x53\x0D\xD1\x98", b"\x55\xA2\x20\x47", b"\x47\x20\xA2\x55",
	b"\xE5\xA6\x3A\x33", b"\x33\x3A\xA6\xE5", b"\xBE\xC6\x50\x61", b"\x61\x50\xC6\xBE",
	b"\xBC\x11\x83\x70", b"\x70\x83\x11\xBC", b"\x0E\x9D\x0E\x75", b"\x75\x0E\x9D\x0E"
];

pub const VERSIONS: [&str; 15] = [
	"v1.0", "v1.1", "v1.2", "v1.3", "v1.3.1",
	"Kiosk", "v1.3.3", "v1.3.4", "v1.4", "v1.5",
	"v1.5*", "v1.6", "v1.6*", "v1.6**","v1.6***"
];

pub const FILE_SIZES: [u64; 15] = [
	896976, 897160, 897112, 907824, 907824, 916576,
	1020648, 1020648, 1027208, 1027208, 1027248, 1027216,
	1027216, 1027216, 1027216
];

pub const HEADERS_2: [&[u8; 2]; 12] = [
	b"\x24\xE2", b"\x24\xEE", b"\x25\x88", b"\x29\xC0", b"\x2A\x46", b"\x2F\x8E",
	b"\x3E\xF8", b"\x3E\xF9", b"\x47\x1A", b"\x47\x1B", b"\x47\x1B", b"\x47\x1E"
];

pub const HEADERS_3: [&[u8; 3]; 3] = [
	b"\x0F\x42\x3D", b"\x0F\x42\x3E", b"\x0F\x42\x3F"
];
