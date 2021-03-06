use super::{Region, Size, Rect};

pub trait Render {
    // Layout utilities.
    fn measure_text(&mut self, text: &str) -> Size;

    // Visitor pattern for rendering.
    fn render_text(&mut self, region: &Region, text: &str);
    fn render_button(&mut self, region: &Region, texture: &'static str);
    fn render_toggle_button(&mut self, region: &Region, is_pressed: bool, is_enabled: bool);
    fn render_window(&mut self, region: &Region);
    fn render_input_highlight(&mut self, rect: &Rect);
}
