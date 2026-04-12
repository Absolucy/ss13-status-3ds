use std::fmt::Display;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Color {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Default,
}

impl Color {
	pub fn fg_code(&self) -> &'static str {
		match self {
			Color::Black => "30",
			Color::Red => "31",
			Color::Green => "32",
			Color::Yellow => "33",
			Color::Blue => "34",
			Color::Magenta => "35",
			Color::Cyan => "36",
			Color::White => "37",
			Color::Default => "39",
		}
	}

	pub fn bg_code(&self) -> &'static str {
		match self {
			Color::Black => "40",
			Color::Red => "41",
			Color::Green => "42",
			Color::Yellow => "43",
			Color::Blue => "44",
			Color::Magenta => "45",
			Color::Cyan => "46",
			Color::White => "47",
			Color::Default => "49",
		}
	}
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiBuilder {
	fg: Option<Color>,
	bg: Option<Color>,
	bold: bool,
	dim: bool,
	italics: bool,
}

impl AnsiBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn fg(mut self, color: Color) -> Self {
		self.fg = Some(color);
		self
	}

	pub fn bg(mut self, color: Color) -> Self {
		self.bg = Some(color);
		self
	}

	pub fn bold(mut self) -> Self {
		self.bold = true;
		self
	}

	pub fn dim(mut self) -> Self {
		self.dim = true;
		self
	}

	pub fn italics(mut self) -> Self {
		self.italics = true;
		self
	}

	pub fn finish(self) -> String {
		let mut operators = Vec::<&'static str>::with_capacity(5);
		if let Some(fg) = self.fg {
			operators.push(fg.fg_code());
		}
		if let Some(bg) = self.bg {
			operators.push(bg.bg_code());
		}
		if self.bold {
			operators.push("1");
		}
		if self.dim {
			operators.push("2");
		}
		if self.italics {
			operators.push("3");
		}
		format!("\x1b[{}m", operators.join(";"))
	}
}

impl Display for AnsiBuilder {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.finish())
	}
}

#[inline]
pub fn ansi() -> AnsiBuilder {
	AnsiBuilder::default()
}

#[inline]
pub fn fg(color: Color) -> String {
	ansi().fg(color).finish()
}

#[inline]
pub fn bg(color: Color) -> String {
	ansi().bg(color).finish()
}
