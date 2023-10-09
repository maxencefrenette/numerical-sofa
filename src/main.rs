#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

use geo::{coord, polygon, Area, BooleanOps, Coord, Point, Polygon, Rect, Rotate, Translate};
use gui::Gui;
use optimization::{ArmijoLineSearch, Func, GradientDescent, Minimizer, NumericalDifferentiation};

fn main() -> Result<(), eframe::Error> {
    let num_points = 4;
    let max_iterations = 100;

    let untrimmed_sofa = Rect::new((-2.0, -0.5), (2.0, 0.5)).to_polygon();
    let hallway = polygon![
        (x: 1.0, y: 1.0),
        (x: 1.0, y: -4.0),
        (x: 0.0, y: -4.0),
        (x: 0.0, y: 0.0),
        (x: -4.0, y: 0.0),
        (x: -4.0, y: 1.0),
    ];

    let function = NumericalDifferentiation::new(Func(|x: &[f64]| {
        let trajectory = make_trajectory(x);
        println!("{:?}", trajectory);
        let sofa = trim_sofa(&untrimmed_sofa, &hallway, &trajectory);
        -sofa.unsigned_area()
    }));

    let minimizer = GradientDescent::new()
        .line_search(ArmijoLineSearch::new(0.5, 0.01, 0.5))
        .max_iterations(Some(max_iterations));
    let solution = minimizer.minimize(&function, vec![0.0; 2 * num_points]);
    let trajectory = make_trajectory(&solution.position);
    let sofa = trim_sofa(&untrimmed_sofa, &hallway, &trajectory);

    let gui = Gui {
        sofa,
        hallway,
        trajectory,
    };
    gui.run()
}

fn make_trajectory(x: &[f64]) -> Vec<Coord> {
    assert!(x.len() % 2 == 0);
    let mut trajectory = vec![];

    trajectory.push(coord!(x: -2.0, y: 0.5));
    for i in 0..x.len() / 2 {
        let mut coord = coord!(x: x[2 * i], y: x[2 * i + 1]);
        coord.x = coord.x.clamp(-1.0, 1.0);
        coord.y = coord.y.clamp(-1.0, 1.0);
        trajectory.push(coord);
    }

    trajectory.push(coord!(x: 0.5, y: -2.0));
    trajectory
}

/// Trim the sofa to fit the hallway given a trajectory
fn trim_sofa(sofa: &Polygon, hallway: &Polygon, trajectory: &Vec<Coord>) -> Polygon {
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
