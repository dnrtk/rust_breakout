use eframe::egui::*;

// Bar size
pub const BAR_WIDTH: f32 = 70.0;
pub const BAR_HEIGHT: f32 = 10.0;
// Bar position
const BAR_START_POS_X_INIT: f32 = 200.0;
const BAR_START_POS_Y_INIT: f32 = 500.0;
// Bar color
const BAR_COLOR_INIT: Color32 = Color32::LIGHT_GRAY;
const BAR_BORDER_COLOR_INIT: Color32 = Color32::GRAY;

pub struct Bar {
    pos_min: Pos2,
    pos_max: Pos2,
    color: Color32,
    border_color: Color32,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            pos_min: pos2(BAR_START_POS_X_INIT, BAR_START_POS_Y_INIT),
            pos_max: pos2(BAR_START_POS_X_INIT + BAR_WIDTH, BAR_START_POS_Y_INIT + BAR_HEIGHT),
            color: BAR_COLOR_INIT,
            border_color: BAR_BORDER_COLOR_INIT,
        }
    }

    pub fn repaint(&mut self, painter: &Painter) {
        painter.rect(
            Rect::from_min_max(self.pos_min, self.pos_max),
            1.0,
            self.color,
            Stroke::new(1.0, self.border_color),
        );
    }
}
