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
// Bar speed
const BAR_SPEED_INIT: f32 = 3.0;

pub enum DIRECTION {
    RIGHT,
    LEFT,
}


pub struct Bar {
    pub pos_min: Pos2,
    pub pos_max: Pos2,
    color: Color32,
    border_color: Color32,
    speed: f32,
    left_min: f32,
    right_max: f32,
}

impl Bar {
    pub fn new(left_min: f32, right_max: f32) -> Self {
        Self {
            pos_min: pos2(BAR_START_POS_X_INIT, BAR_START_POS_Y_INIT),
            pos_max: pos2(BAR_START_POS_X_INIT + BAR_WIDTH, BAR_START_POS_Y_INIT + BAR_HEIGHT),
            color: BAR_COLOR_INIT,
            border_color: BAR_BORDER_COLOR_INIT,
            speed: BAR_SPEED_INIT,
            left_min,
            right_max,
        }
    }

    pub fn update_position(&mut self, direction: DIRECTION) {
        match direction {
            DIRECTION::RIGHT => {
                self.pos_max.x += self.speed;
                if self.pos_max.x > self.right_max {
                    self.pos_max.x = self.right_max;
                }
                self.pos_min.x = self.pos_max.x - BAR_WIDTH;
            },
            DIRECTION::LEFT => {
                self.pos_min.x -= self.speed;
                if self.pos_min.x < self.left_min {
                    self.pos_min.x = self.left_min;
                }
                self.pos_max.x = self.pos_min.x + BAR_WIDTH;
            }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let bar_ins = Bar::new(0.0, 500.0);
        assert_eq!(bar_ins.pos_min, pos2(BAR_START_POS_X_INIT, BAR_START_POS_Y_INIT));
        assert_eq!(bar_ins.pos_max, pos2(BAR_START_POS_X_INIT + BAR_WIDTH, BAR_START_POS_Y_INIT + BAR_HEIGHT));
    }

    #[test]
    fn test_init2() {
        let bar_ins = Bar::new(0.0, 500.0);
        assert_eq!(bar_ins.left_min, 0.0);
        assert_eq!(bar_ins.right_max, 500.0);
    }
}
