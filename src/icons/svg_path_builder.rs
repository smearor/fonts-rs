//! SVG path data builder implementing `ttf_parser::OutlineBuilder`.

use std::fmt::Write;

use ttf_parser::OutlineBuilder;

/// SVG path data builder implementing `ttf_parser::OutlineBuilder`.
///
/// Collects SVG path commands (`M`, `L`, `Q`, `C`, `Z`) as the font
/// outline is traversed, then exposes the accumulated path string.
pub struct SvgPathBuilder {
    pub path: String,
}

impl SvgPathBuilder {
    pub fn new() -> Self {
        Self { path: String::new() }
    }
}

impl OutlineBuilder for SvgPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(self.path, "M {} {} ", x, y).unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(self.path, "L {} {} ", x, y).unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(self.path, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(self.path, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    }

    fn close(&mut self) {
        self.path.push_str("Z ");
    }
}
