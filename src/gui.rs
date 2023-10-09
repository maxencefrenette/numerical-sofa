use eframe::egui::{self, Pos2};
use geo::{AffineOps, AffineTransform, Coord, Point, Polygon, Rotate, Translate};

pub struct Gui {
    pub sofa: Polygon<f64>,
    pub hallway: Polygon<f64>,
    pub sofa_positions: Vec<Coord>,
}

impl Gui {
    pub fn run(self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1280.0, 720.0)),
            ..Default::default()
        };

        eframe::run_native(
            "Numerical Sofa Results Viewer",
            options,
            Box::new(|_| Box::new(self)),
        )
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 700.0;

            let mut step = 0;
            ui.add(
                egui::Slider::new(&mut step, 0..=(self.sofa_positions.len() - 1)).show_value(false),
            );

            let (response, p) = ui.allocate_painter(
                egui::Vec2::new(700.0, 700.0),
                egui::Sense::focusable_noninteractive(),
            );
            let rect = response.rect;

            let scale = 150.0;
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
            let sofa_position = self.sofa_positions[step];
            let sofa_rotation = -90.0 * (step as f64) / (self.sofa_positions.len() as f64 - 1.0);
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
        });
    }
}
