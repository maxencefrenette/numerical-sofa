#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

use geo::{coord, polygon, Rect};
use gui::Gui;

fn main() -> Result<(), eframe::Error> {
    let sofa = Rect::new((-1.0, -0.5), (1.0, 0.5)).to_polygon();
    let hallway = polygon![
        (x: 1.0, y: 1.0),
        (x: 1.0, y: -2.0),
        (x: 0.0, y: -2.0),
        (x: 0.0, y: 0.0),
        (x: -2.0, y: 0.0),
        (x: -2.0, y: 1.0),
    ];
    let sofa_positions = vec![
        coord!(x: -1.0, y: 0.5),
        coord!(x: 0.5, y: 0.5),
        coord!(x: 0.5, y: -1.0),
    ];

    let gui = Gui {
        sofa,
        hallway,
        sofa_positions,
    };
    gui.run()
}
