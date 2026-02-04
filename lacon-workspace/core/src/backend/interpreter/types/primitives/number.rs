use strum_macros::{AsRefStr, VariantNames};

#[derive(VariantNames, AsRefStr)]
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
	// Float16(f16),
	Float32(f32),
	Float64(f64),
	// Float128(f128),
}

impl Number {
	pub fn match_type_name(&self, name: &str) -> bool {
		self.as_ref() == name
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_match_if_string_equals_to_subtype_name() {
		let num = Number::Int32(42);
		let result1 = num.match_type_name("Int32");
		println!("Comparing '{}' with 'Int32': {}", num.as_ref(), result1);
		assert!(result1);

		let result2 = num.match_type_name("Int64");
		println!("Comparing '{}' with 'Int64': {}", num.as_ref(), result2);
		assert!(!result2);

		let stringname = "Float64";
		let num2 = Number::Float64(3.14);
		let result3 = num2.match_type_name(stringname);
		println!("Comparing '{}' with '{}': {}", num2.as_ref(), stringname, result3);
		assert!(result3);

		let result4 = num2.match_type_name("Int16");
		println!("Comparing '{}' with 'Int16': {}", num2.as_ref(), result4);
		assert!(!result4);
	}
}
