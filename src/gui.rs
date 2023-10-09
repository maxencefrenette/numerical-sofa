use eframe::egui::{self, Pos2, Ui};
use geo::{AffineOps, AffineTransform, Area, Coord, Point, Polygon, Rotate, Translate};

pub struct Gui {
    pub sofa: Polygon<f64>,
    pub hallway: Polygon<f64>,
    pub trajectory: Vec<Coord>,
}

impl Gui {
    pub fn run(self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1020.0, 1050.0)),
            ..Default::default()
        };

        eframe::run_native(
            "Numerical Sofa Results Viewer",
            options,
            Box::new(|_| Box::new(self)),
        )
    }

    fn update_sofa(&self, ui: &mut Ui, step: usize) {
        let (response, p) = ui.allocate_painter(
            egui::Vec2::new(1000.0, 1000.0),
            egui::Sense::focusable_noninteractive(),
        );
        let rect = response.rect;

        let scale = 125.0;
        let view_transform = AffineTransform::identity()
            .translated(rect.center().x as f64, rect.center().y as f64)
            .scaled(scale, -scale, (0.0, 0.0));

        // Make background white
        p.rect_filled(rect, 0.0, egui::Color32::WHITE);

        // draw hallway
        p.add(egui::Shape::closed_line(
            self.hallway
                .affine_transform(&view_transform)
                .exterior()
                .coords()
                .map(|c| Pos2::new(c.x as f32, c.y as f32))
                .collect(),
            egui::Stroke::new(3.0, egui::Color32::BLACK),
        ));

        // draw sofa
        let sofa_position = self.trajectory[step];
        let sofa_rotation = -90.0 * (step as f64) / (self.trajectory.len() as f64 - 1.0);
        p.add(egui::Shape::closed_line(
            self.sofa
                .rotate_around_point(sofa_rotation, Point::new(0.0, 0.0))
                .translate(sofa_position.x, sofa_position.y)
                .affine_transform(&view_transform)
                .exterior()
                .coords()
                .map(|c| Pos2::new(c.x as f32, c.y as f32))
                .collect(),
            egui::Stroke::new(2.0, egui::Color32::RED),
        ));
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Area: {:.5}", self.sofa.unsigned_area()));

            ui.style_mut().spacing.slider_width = 700.0;

            let mut step = 0;
            ui.add(egui::Slider::new(&mut step, 0..=(self.trajectory.len() - 1)).show_value(false));

            self.update_sofa(ui, step);
        });
    }
}
