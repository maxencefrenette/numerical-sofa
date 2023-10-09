#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

use geo::{coord, polygon, BooleanOps, Coord, Point, Polygon, Rect, Rotate, Translate};
use gui::Gui;

fn main() -> Result<(), eframe::Error> {
    let untrimmed_sofa = Rect::new((-1.0, -0.5), (1.0, 0.5)).to_polygon();
    let hallway = polygon![
        (x: 1.0, y: 1.0),
        (x: 1.0, y: -2.0),
        (x: 0.0, y: -2.0),
        (x: 0.0, y: 0.0),
        (x: -2.0, y: 0.0),
        (x: -2.0, y: 1.0),
    ];
    let trajectory = vec![
        coord!(x: -1.0, y: 0.5),
        coord!(x: 0.5, y: 0.5),
        coord!(x: 0.5, y: -1.0),
    ];

    let sofa = trim_sofa(&untrimmed_sofa, &hallway, trajectory.clone());

    let gui = Gui {
        sofa,
        hallway,
        trajectory,
    };
    gui.run()
}

/// Trim the sofa to fit the hallway given a trajectory
fn trim_sofa(sofa: &Polygon, hallway: &Polygon, trajectory: Vec<Coord>) -> Polygon {
    let mut sofa = sofa.clone();

    for (i, position) in trajectory.iter().enumerate() {
        let rotation = 90.0 * (i as f64) / (trajectory.len() as f64 - 1.0);

        let rotated_hallway = hallway
            .translate(-position.x, -position.y)
            .rotate_around_point(rotation, Point::new(0.0, 0.0));

        sofa = sofa
            .intersection(&rotated_hallway)
            .iter()
            .next()
            .unwrap()
            .clone();
    }

    sofa
}
