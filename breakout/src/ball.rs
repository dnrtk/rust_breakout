use std::f64::consts::PI;
use eframe::egui::*;

// Ball parameters
const BALL_RADIUS: f32 = 5.0;
const BALL_START_POS_X_INIT: f32 = 250.0;
const BALL_START_POS_Y_INIT: f32 = 495.0;
const BALL_SPEED_INIT: f64 = 5.0;
const BALL_DEG_INIT: f64 = PI/4.0;
const BALL_COLOR_INIT: Color32 = Color32::LIGHT_RED;

pub struct Ball {
    pos: Pos2,
    speed: f64,
    deg: f64,
    pos_delta: Vec2,
    color: Color32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: pos2(BALL_START_POS_X_INIT, BALL_START_POS_Y_INIT),
            pos_delta: vec2(0.0, 0.0),
            speed: BALL_SPEED_INIT,
            deg: BALL_DEG_INIT,
            color: BALL_COLOR_INIT,
        }
    }

    pub fn updateDeg(&mut self, deg: f64) {
        // 引数degは進行方向を下方向を0として半時計回りに0-360度までを指定する
        self.deg = deg;
        let rad = deg / 180.0 * PI;
        self.pos_delta = vec2(
            (self.speed * rad.sin()) as f32,
            (self.speed * rad.cos()) as f32
        );
    }

    pub fn updatePosition(&mut self) {
        // TODO: 衝突判定を追加
        self.pos += self.pos_delta;
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