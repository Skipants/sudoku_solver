pub fn bitmask_to_single_value(bitmask: i32) -> i32 {
	match bitmask {
		0x001 => 1,
		0x002 => 2,
		0x004 => 3,
		0x008 => 4,
		0x010 => 5,
		0x020 => 6,
		0x040 => 7,
		0x080 => 8,
		0x100 => 9,
		_ => 0,
	}
}

pub fn single_value_to_bitmask(value: i32) -> i32 {
	match value {
		1 => 0x001,
		2 => 0x002,
		3 => 0x004,
		4 => 0x008,
		5 => 0x010,
		6 => 0x020,
		7 => 0x040,
		8 => 0x080,
		9 => 0x100,
		_ => 0x1FF,
	}
}
