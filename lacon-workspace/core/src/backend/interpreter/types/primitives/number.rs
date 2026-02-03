// use f128::f128;
// use half::{f8, f16};

#[feature(f16_and_f128)]
pub enum Number {
	Number,
	Int(i64),
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
	Int128(i128),
	UInt8(u8),
	UInt16(u16),
	UInt32(u32),
	UInt64(u64),
	UInt128(u128),
	Float(f64),
	Float16(f16),
	Float32(f32),
	Float64(f64),
	Float128(f128),
}
