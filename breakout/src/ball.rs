use std::f64::consts::PI;
use eframe::egui::*;

use super::field;
use super::block;
use super::bar;

// Ball parameters
const BALL_RADIUS: f32 = 5.0;
const BALL_START_POS_X_INIT: f32 = 250.0;
const BALL_START_POS_Y_INIT: f32 = 495.0;
const BALL_SPEED_INIT: f64 = 5.0;
const BALL_DEG_INIT: f64 = 135.0;
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

    pub fn update_deg(&mut self, deg: f64) {
        // 引数degは進行方向を下方向を0として半時計回りに0-360度までを指定する
        self.deg = deg % 360.0;
        let rad = deg / 180.0 * PI;
        self.pos_delta = vec2(
            (self.speed * rad.sin()) as f32,
            (self.speed * rad.cos()) as f32
        );
    }

    pub fn update_position(&mut self, field: &field::Field, block_list: &mut Vec<block::Block>, bar: &bar::Bar) {
        // 移動先座標を計算
        let mut next_pos = self.pos + self.pos_delta;

        // barとの衝突判定
        if (bar.pos_min.x <= next_pos.x) && (next_pos.x <= bar.pos_max.x) {
            if (next_pos.y <= bar.pos_min.y) && ((bar.pos_min.y - next_pos.y) <= BALL_RADIUS) {
                // 上部接触判定
                next_pos.y = bar.pos_min.y - BALL_RADIUS;
                let next_deg = (360.0 + (180.0 - self.deg)) % 360.0;
                self.update_deg(next_deg * 0.95);
            }
        }

        // fieldとの衝突判定
        if (field.pos_min.x <= next_pos.x) && (next_pos.x <= field.pos_max.x) {
            if (next_pos.y - field.pos_min.y) <= BALL_RADIUS {
                // 上部接触判定
                next_pos.y = field.pos_min.y + BALL_RADIUS;
                let next_deg = (180.0 + (360.0 - self.deg)) % 360.0;
                self.update_deg(next_deg);
            }
            else if (field.pos_max.y - next_pos.y) <= BALL_RADIUS {
                // 下部接触判定
                self.speed = 0.0;
                self.update_deg(BALL_DEG_INIT);
            }
        }
        else if (field.pos_min.y <= next_pos.y) && (next_pos.y <= field.pos_max.y) {
            if (next_pos.x - field.pos_min.x) <= BALL_RADIUS {
                // 左部接触判定
                next_pos.x = field.pos_min.x + BALL_RADIUS;
                let next_deg = (90.0 + (270.0 - self.deg)) % 360.0;
                self.update_deg(next_deg);
            }
            else if (field.pos_max.x - next_pos.x) < BALL_RADIUS {
                // 右部接触判定
                next_pos.x = field.pos_max.x - BALL_RADIUS;
                let next_deg = (270.0 + (90.0 - self.deg)) % 360.0;
                self.update_deg(next_deg);
            }
        }

        // blockとの衝突判定
        let mut del_index: usize = usize::MAX;
        for index in 0..block_list.len() {
            let pos_min = block_list[index].pos_min;
            let pos_max = block_list[index].pos_max;

            if (pos_min.x <= next_pos.x) && (next_pos.x <= pos_max.x) {
                if (next_pos.y <= pos_min.y) && ((pos_min.y - next_pos.y) <= BALL_RADIUS) {
                    // 上部接触判定
                    next_pos.y = pos_min.y - BALL_RADIUS;
                    let next_deg = (360.0 + (180.0 - self.deg)) % 360.0;
                    self.update_deg(next_deg);
                    del_index = index;
                    break;
                }
                else if (pos_max.y <= next_pos.y) && ((next_pos.y - pos_max.y) <= BALL_RADIUS) {
                    // 下部接触判定
                    next_pos.y = pos_max.y + BALL_RADIUS;
                    let next_deg = (180.0 + (360.0 - self.deg)) % 360.0;
                    self.update_deg(next_deg);
                    del_index = index;
                    break;
                }
            }
            else if (pos_min.y <= next_pos.y) && (next_pos.y <= pos_max.y) {
                if (next_pos.x <= pos_min.x) && ((pos_min.x - next_pos.x) <= BALL_RADIUS) {
                    // 左部接触判定
                    next_pos.x = pos_min.x - BALL_RADIUS;
                    let next_deg = (270.0 + (90.0 - self.deg)) % 360.0;
                    self.update_deg(next_deg);
                    del_index = index;
                    break;
                }
                else if (pos_max.x <= next_pos.x) && ((next_pos.x - pos_max.x) <= BALL_RADIUS) {
                    // 右部接触判定
                    next_pos.x = pos_max.x + BALL_RADIUS;
                    let next_deg = (90.0 + (270.0 - self.deg)) % 360.0;
                    self.update_deg(next_deg);
                    del_index = index;
                    break;
                }
            }
        }
        // 接触したブロックを削除
        if del_index != usize::MAX {
            block_list.remove(del_index);
        }

        // 座標更新
        self.pos = next_pos;
    }

    pub fn repaint(&mut self, painter: &Painter) {
        painter.circle(
            self.pos,
            BALL_RADIUS,
            self.color,
            Stroke::new(1.0, self.color),
        );
    }

    pub fn isStopped(&mut self) -> bool {
        return self.speed == 0.0;
    }
}