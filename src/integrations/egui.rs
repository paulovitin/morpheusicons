#[cfg(feature = "egui")]
use egui::{pos2, Color32, Pos2, Sense, Stroke, Ui, Vec2};

#[cfg(feature = "egui")]
use crate::animation::{DrawCommand, MorphController};

/// egui painter integration for MorpheusIcons.
#[cfg(feature = "egui")]
pub fn paint_morph_icon(
    ui: &mut Ui,
    controller: &MorphController,
    size: Vec2,
    color: Color32,
    stroke_width: f32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let commands = controller.current_draw_commands();
        let scale_x = rect.width() / 24.0;
        let scale_y = rect.height() / 24.0;

        let to_screen = |p: crate::geometry::Point| -> Pos2 {
            pos2(rect.min.x + p.x * scale_x, rect.min.y + p.y * scale_y)
        };

        let stroke = Stroke::new(stroke_width, color);
        let painter = ui.painter();

        let mut current_points = Vec::new();

        for cmd in commands {
            match cmd {
                DrawCommand::MoveTo(p) => {
                    if current_points.len() >= 2 {
                        painter.add(egui::epaint::PathShape::line(
                            std::mem::take(&mut current_points),
                            stroke,
                        ));
                    }
                    current_points = vec![to_screen(p)];
                }
                DrawCommand::LineTo(p) => {
                    current_points.push(to_screen(p));
                }
                DrawCommand::CubicTo { ctrl1, ctrl2, end } => {
                    let p0 = *current_points.last().unwrap_or(&rect.min);
                    let c1 = to_screen(ctrl1);
                    let c2 = to_screen(ctrl2);
                    let p3 = to_screen(end);

                    let bezier = egui::epaint::CubicBezierShape::from_points_stroke(
                        [p0, c1, c2, p3],
                        false,
                        Color32::TRANSPARENT,
                        stroke,
                    );
                    painter.add(bezier);
                    current_points = vec![p3];
                }
                DrawCommand::Close => {
                    if let Some(&first) = current_points.first() {
                        current_points.push(first);
                        painter.add(egui::epaint::PathShape::line(
                            std::mem::take(&mut current_points),
                            stroke,
                        ));
                    } else {
                        current_points.clear();
                    }
                }
            }
        }

        if current_points.len() >= 2 {
            painter.add(egui::epaint::PathShape::line(current_points, stroke));
        }
    }

    response
}
