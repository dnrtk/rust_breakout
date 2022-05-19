use std::f64::consts::PI;
use eframe::egui::*;

// Ball parameters
const BALL_RADIUS: f32 = 5.0;
const BALL_START_POS_X_INIT: f32 = 250.0;
const BALL_START_POS_Y_INIT: f32 = 500.0;
const BALL_SPEED_INIT: f32 = 10.0;
const BALL_DEG_INIT: f64 = PI/4.0;
const BALL_COLOR_INIT: Color32 = Color32::LIGHT_RED;

pub struct Ball {
    pos: Pos2,
    speed: f32,
    deg: f64,
    color: Color32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: pos2(BALL_START_POS_X_INIT, BALL_START_POS_Y_INIT),
            speed: BALL_SPEED_INIT,
            deg: BALL_DEG_INIT,
            color: BALL_COLOR_INIT,
        }
    }

    pub fn updatePosition(&mut self) {
        // TODO: 衝突判定を追加
        self.pos.x -= 1.0;
        self.pos.y -= 1.0;
    }

    pub fn repaint(&mut self, painter: &Painter) {
        painter.circle(
            self.pos,
            BALL_RADIUS,
            self.color,
            Stroke::new(1.0, self.color),
        );
    }
}