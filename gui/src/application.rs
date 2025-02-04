use eframe::egui;
use eframe::egui::{Frame, Rect, Visuals};
use crate::notifications::NotificationPanel;

pub struct App {
    state: AppState,
}

pub struct AppState {
    is_menu_open: bool,
    notify_panel: NotificationPanel,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_menu_open: true,
            notify_panel: NotificationPanel::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    fn title_bar(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_theme_preference_switch(ui);

        ui.separator();

        ui.toggle_value(&mut self.state.is_menu_open, "🗖 Menu");

        let notification_label = if self.state.notify_panel.notifications.is_empty() {
            "🔔 Notifications"
        }
        else {
            &format!("🔔 Notifications ({})", self.state.notify_panel.notifications.len())
        };

        ui.with_layout(egui::Layout::default().with_cross_align(egui::Align::RIGHT), |ui| {
            ui.toggle_value(&mut self.state.notify_panel.is_open, notification_label);
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let window_rect = ctx.input(|i| i.viewport().outer_rect);
        if window_rect.is_none() { return; }

        let window_rect = window_rect.unwrap();

        egui::TopBottomPanel::top("app_top_panel")
            .frame(Frame::default().inner_margin(8.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.visuals_mut().button_frame = false;
                    self.title_bar(ui, frame);
                })
            });

        let panel_frame = Frame::default()
            .inner_margin(8.0)
            .fill(ctx.style().visuals.panel_fill);

        egui::SidePanel::left("left_side_main_menu")
            .frame(panel_frame)
            .resizable(false)
            .exact_width(window_rect.width() * 0.15)
            .show_animated(ctx, self.state.is_menu_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 katabasis");
                });

                ui.separator();
            });

        egui::SidePanel::right("right_side_notification_menu")
            .frame(panel_frame)
            .resizable(true)
            .default_width(window_rect.width() * 0.20)
            .show_animated(ctx, self.state.notify_panel.is_open, |ui| {
                self.state.notify_panel.ui(ui, frame);
            });
    }

    fn clear_color(&self, visuals: &Visuals) -> [f32; 4] {
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );

        egui::Color32::from(color).to_normalized_gamma_f32()
    }
}
