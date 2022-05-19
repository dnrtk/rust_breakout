use eframe::egui;
use eframe::egui::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    ball_pos: Pos2,
    block_list: Vec<Pos2>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            ball_pos: pos2(200.0, 500.0),
            block_list: Vec::new(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            self.ball_pos.y -= 1.0;

            draw_field(painter);
            draw_block(painter, pos2(100.0, 100.0));
            draw_block(painter, pos2(100.0, 200.0));
            draw_block(painter, pos2(100.0, 300.0));
            draw_block(painter, pos2(100.0, 400.0));
            draw_ball(painter, self.ball_pos);
        });

        // 再描画
        ctx.request_repaint();
    }
}

// Game field size
const FIELD_WIDTH: f32 = 500.0;
const FIELD_HEIGHT: f32 = 600.0;
// Block size
const BLOCK_WIDTH: f32 = 100.0;
const BLOCK_HEIGHT: f32 = 20.0;
// Block start position
const BLOCK_START_POS_X: f32 = 50.0;
const BLOCK_START_POS_Y: f32 = 50.0;
// Ball size
const BALL_RADIUS: f32 = 5.0;

fn draw_field(painter: &Painter) {
    painter.rect(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(FIELD_WIDTH, FIELD_HEIGHT)),
        0.0,
        Color32::TRANSPARENT,
        Stroke::new(2.0, Color32::BLACK),
    );
}

fn draw_block(painter: &Painter, pos: Pos2) {
    let size = vec2(BLOCK_WIDTH, BLOCK_HEIGHT);
    painter.rect(
        Rect::from_min_size(pos, size),
        0.0,
        Color32::LIGHT_BLUE,
        Stroke::new(1.0, Color32::BLUE),
    );
}

fn draw_ball(painter: &Painter, pos: Pos2) {
    painter.circle(
        pos,
        BALL_RADIUS,
        Color32::LIGHT_RED,
        Stroke::new(1.0, Color32::RED),
    );
}
