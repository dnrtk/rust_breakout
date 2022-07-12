use eframe::egui::*;

// Block size
pub const BLOCK_WIDTH: f32 = 100.0;
pub const BLOCK_HEIGHT: f32 = 20.0;
const BLOCK_COLOR_INIT: Color32 = Color32::BLUE;
const BLOCK_BORDER_COLOR_INIT: Color32 = Color32::LIGHT_BLUE;

pub struct Block {
    pub pos_min: Pos2,
    pub pos_max: Pos2,
    color: Color32,
    border_color: Color32,
}

impl Block {
    pub fn new(start_x: f32, start_y: f32) -> Self {
        Self {
            pos_min: pos2(start_x, start_y),
            pos_max: pos2(start_x + BLOCK_WIDTH, start_y + BLOCK_HEIGHT),
            color: BLOCK_COLOR_INIT,
            border_color: BLOCK_BORDER_COLOR_INIT,
        }
    }

    pub fn repaint(&mut self, painter: &Painter) {
        painter.rect(
            Rect::from_min_max(self.pos_min, self.pos_max),
            0.0,
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
        let block_ins = Block::new(1.2, 3.4);
        assert_eq!(block_ins.pos_min, pos2(1.2, 3.4));
    }

    #[test]
    fn test_init2() {
        let block_ins = Block::new(1.2, 3.4);
        assert_eq!(block_ins.pos_max, pos2(1.2+BLOCK_WIDTH, 3.4+BLOCK_HEIGHT));
    }
}
