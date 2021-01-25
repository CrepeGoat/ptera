struct MathExpr;

impl FractalParser for MathExpr {
	// These should be default-implemented functions the user doesn't have to touch
	fn new() -> Self;  // This would be edited if the parser takes inputs
	fn init(&mut self: Self) {  // This would need to be called explicitly by an edited `::new()` method
		self.core = ParserCore::new();
		self.build_onto_core(self.core);
	}
	fn build_onto_core(&mut self) {
		if *self in self.core.parser_refs {
			return;
		}
		self.define_format().build_onto_core(self.core);
	}

	// This is the main function the user should define
	fn define_format() -> impl FractalParser {
		Alt(
			Digits(),
			Seq(Str("("), &self, Str(")")),
			Seq(&self, Alt(Str(" + "), Str(" * ")), &self),
		)
	}
}