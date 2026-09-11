//! SVG path data builder implementing `skrifa::outline::OutlinePen`.

use std::fmt::Write;

use skrifa::outline::OutlinePen;

/// SVG path data builder implementing `skrifa::outline::OutlinePen`.
///
/// Collects SVG path commands (`M`, `L`, `Q`, `C`, `Z`) as the font
/// outline is traversed, then exposes the accumulated path string.
/// Also tracks the bounding box of all points visited.
pub struct SvgPathBuilder {
    /// Accumulated SVG path data string.
    pub path: String,
    /// Minimum X coordinate of the bounding box.
    pub x_min: f32,
    /// Minimum Y coordinate of the bounding box.
    pub y_min: f32,
    /// Maximum X coordinate of the bounding box.
    pub x_max: f32,
    /// Maximum Y coordinate of the bounding box.
    pub y_max: f32,
}

impl SvgPathBuilder {
    /// Creates a new empty `SvgPathBuilder`.
    pub fn new() -> Self {
        Self {
            path: String::new(),
            x_min: f32::MAX,
            y_min: f32::MAX,
            x_max: f32::MIN,
            y_max: f32::MIN,
        }
    }

    fn update_bounds(&mut self, x: f32, y: f32) {
        self.x_min = self.x_min.min(x);
        self.y_min = self.y_min.min(y);
        self.x_max = self.x_max.max(x);
        self.y_max = self.y_max.max(y);
    }

    /// Returns the bounding box as `(x_min, y_min, x_max, y_max)` if any
    /// points were visited, or `None` if the path is empty.
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        if self.x_min > self.x_max || self.y_min > self.y_max {
            None
        } else {
            Some((self.x_min, self.y_min, self.x_max, self.y_max))
        }
    }
}

impl Default for SvgPathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OutlinePen for SvgPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.update_bounds(x, y);
        let _ = write!(self.path, "M {} {} ", x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.update_bounds(x, y);
        let _ = write!(self.path, "L {} {} ", x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.update_bounds(x1, y1);
        self.update_bounds(x, y);
        let _ = write!(self.path, "Q {} {} {} {} ", x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.update_bounds(x1, y1);
        self.update_bounds(x2, y2);
        self.update_bounds(x, y);
        let _ = write!(self.path, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.path.push_str("Z ");
    }
}
