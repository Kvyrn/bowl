use three_d::egui::*;

fn fraction_bar_widget(ui: &mut Ui, fill: f32, color: Color32) -> Response {
    let height = ui.spacing().interact_size.y;

    let (mut rect, response) = ui.allocate_at_least(vec2(96.0, height), Sense::hover());

    if ui.is_rect_visible(rect) {
        let animated_fill = ui
            .ctx()
            .animate_value_with_time(response.id, fill.min(1.0), 0.2);

        let width = rect.width() * animated_fill;
        let radius = 0.1 * rect.height();

        // Background
        ui.painter()
            .rect_filled(rect, radius, Color32::from_gray(50));
        // Fill
        rect.set_width(width);
        ui.painter().rect_filled(rect, radius, color);
    }
    response
}

pub fn fraction_bar(fill: f32, color: Color32) -> impl Widget + 'static {
    move |ui: &mut Ui| fraction_bar_widget(ui, fill, color)
}
