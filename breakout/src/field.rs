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
    pub pos_min: Pos2,
    pub pos_max: Pos2,
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

    pub fn show_gameover(&mut self, painter: &Painter) {
        painter.text(
            pos2(FIELD_WIDTH/2.0, FIELD_HEIGHT/2.0),
            Align2::CENTER_CENTER,
            "GAME OVER!!",
            FontId::proportional(80.0),
            Color32::DARK_RED,
        );
    }

    pub fn show_gameclear(&mut self, painter: &Painter) {
        painter.text(
            pos2(FIELD_WIDTH/2.0, FIELD_HEIGHT/2.0),
            Align2::CENTER_CENTER,
            "GAME CLEAR!!",
            FontId::proportional(80.0),
            Color32::GREEN,
        );
    }
}
