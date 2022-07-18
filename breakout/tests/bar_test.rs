extern crate breakout;
use eframe::egui::*;

#[test]
fn init_test() {
    let bar_instance = breakout::bar::Bar::new(0.0, 500.0);
    assert_eq!(bar_instance.pos_min, pos2(200.0, 500.0));
}

#[test]
fn right_move_test() {
    let mut bar_instance = breakout::bar::Bar::new(0.0, 500.0);
    bar_instance.update_position(breakout::bar::DIRECTION::RIGHT);
    assert_eq!(bar_instance.pos_min, pos2(203.0, 500.0));
    assert_eq!(bar_instance.pos_max, pos2(273.0, 510.0));
}

#[test]
fn left_move_test() {
    let mut bar_instance = breakout::bar::Bar::new(0.0, 500.0);
    bar_instance.update_position(breakout::bar::DIRECTION::LEFT);
    assert_eq!(bar_instance.pos_min, pos2(197.0, 500.0));
    assert_eq!(bar_instance.pos_max, pos2(267.0, 510.0));
}
