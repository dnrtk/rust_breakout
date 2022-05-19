use eframe::egui;
use eframe::egui::*;

mod field;
mod ball;
mod block;
mod bar;

// Block start position
const BLOCK_START_POS_X: f32 = 50.0;
const BLOCK_START_POS_Y: f32 = 50.0;
// Block num
const BLOCK_NUM_X: i32 = 4;
const BLOCK_NUM_Y: i32 = 3;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

// #[derive(Default)]
struct MyEguiApp {
    field: field::Field,
    block_list: Vec<block::Block>,
    ball: ball::Ball,
    bar: bar::Bar,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut ins = Self {
            field: field::Field::new(),
            block_list: Vec::new(),
            ball: ball::Ball::new(),
            bar: bar::Bar::new(),
        };
        for x in 0..BLOCK_NUM_X {
            for y in 0..BLOCK_NUM_Y {
                let pos_x = BLOCK_START_POS_X + (block::BLOCK_WIDTH * (x as f32));
                let pos_y = BLOCK_START_POS_Y + (block::BLOCK_HEIGHT * (y as f32));
                let temp_block = block::Block::new(pos_x, pos_y);
                ins.block_list.push(temp_block);
            }
        }
        return ins;
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // 状態再計算
            self.ball.updatePosition();

            // 再描画
            self.field.repaint(painter);
            for index in 0..self.block_list.len() {
                self.block_list[index].repaint(painter);
            }
            self.ball.repaint(painter);
            self.bar.repaint(painter);

        });

        // 次フレームの再描画要求
        ctx.request_repaint();
    }
}
