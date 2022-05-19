use eframe::egui::*;

// Field size
pub const FIELD_WIDTH: f32 = 500.0;
pub const FIELD_HEIGHT: f32 = 600.0;
// Field position
const FIELD_START_POS_X_INIT: f32 = 0.0;
const FIELD_START_POS_Y_INIT: f32 = 0.0;
// Field color
const FIELD_COLOR_INIT: Color32 = Color32::TRANSPARENT;
const FIELD_BORDER_COLOR_INIT: Color32 = Color32::BLACK;

pub struct Field {
    pos_min: Pos2,
    pos_max: Pos2,
    color: Color32,
    border_color: Color32,
}

impl Field {
    pub fn new() -> Self {
        Self {
            pos_min: pos2(FIELD_START_POS_X_INIT, FIELD_START_POS_Y_INIT),
            pos_max: pos2(FIELD_START_POS_X_INIT + FIELD_WIDTH, FIELD_START_POS_Y_INIT + FIELD_HEIGHT),
            color: FIELD_COLOR_INIT,
            border_color: FIELD_BORDER_COLOR_INIT,
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
