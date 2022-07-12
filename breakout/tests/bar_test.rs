extern crate breakout;
use eframe::egui::*;

#[test]
fn init_test() {
    let bar_instance = breakout::bar::Bar::new(1.2, 3.4);
    assert_eq!(bar_instance.pos_min, pos2(200.0, 500.0));
}
