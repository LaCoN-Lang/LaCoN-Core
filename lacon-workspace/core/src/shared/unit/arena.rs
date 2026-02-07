use bumpalo::Bump;

#[derive(Debug)]
pub struct UnitArena {
	bump: Bump,
}

impl UnitArena {
	pub fn new() -> Self {
		Self { bump: Bump::new() }
	}

	#[inline(always)]
	pub fn alloc_str<'a>(&'a self, s: &str) -> &'a str {
		self.bump.alloc_str(s)
	}
}
