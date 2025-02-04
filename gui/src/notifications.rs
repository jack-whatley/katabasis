use eframe::egui;
use eframe::egui::{Frame, Label, RichText, Sense, UiBuilder, Widget};
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub enum NotificationType {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub level: NotificationType,
}

impl Notification {
    pub fn error(title: &str, body: &str) -> Self {
        Self::new(title, body, NotificationType::Error)
    }

    pub fn warning(title: &str, body: &str) -> Self {
        Self::new(title, body, NotificationType::Warning)
    }

    pub fn info(title: &str, body: &str) -> Self {
        Self::new(title, body, NotificationType::Info)
    }

    fn new<T: Into<String>, B: Into<String>>(
        title: T,
        body: B,
        level: NotificationType
    ) -> Self {
        Self { title: title.into(), body: body.into(), level, }
    }
}

pub struct NotificationPanel {
    pub is_open: bool,
    pub notifications: Vec<Notification>,
    receiver: Option<Receiver<Notification>>,
}

impl Default for NotificationPanel {
    fn default() -> Self {
        Self {
            is_open: false,
            notifications: vec![
                Notification::error("Test Notification 1", "Test notification body this is some desc text."),
                Notification::warning("Test Notification 2", "This is the test notification two body text, may include a warning who knows?"),
            ],
            receiver: None,
        }
    }
}

impl NotificationPanel {
    pub fn set_receiver(&mut self, receiver: Receiver<Notification>) {
        self.receiver = Some(receiver);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.update_notifications();

        ui.horizontal(|ui| {
            ui.label("🔔 Notifications");
        });

        ui.separator();

        let cloned_notifications = self.notifications.clone();

        for i in 0..cloned_notifications.len() {
            let notification = &cloned_notifications[i];
            let response = self.render_notification(ui, notification);

            if response.clicked() {
                self.notifications.remove(i);
            }
        }
    }

    fn update_notifications(&mut self) {
        if let Some(receiver) = &mut self.receiver {
            match receiver.try_recv() {
                Ok(notification) => {
                    self.notifications.push(notification);
                }
                Err(_) => { /* TODO: Log here... */ }
            }
        }
    }

    fn render_notification(&self, ui: &mut egui::Ui, notification: &Notification) -> egui::Response {
        ui.scope_builder(
            UiBuilder::new()
                .id_salt("interactive_notification")
                .sense(Sense::click()),
            |ui| {
                let response = ui.response();
                let visuals = ui.style().interact(&response);
                let text_colour = visuals.text_color();

                Frame::canvas(ui.style())
                    .fill(visuals.bg_fill.gamma_multiply(0.30))
                    .stroke(visuals.bg_stroke)
                    .inner_margin(ui.spacing().menu_margin)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        Label::new(
                            RichText::new(&notification.title)
                                .color(text_colour)
                                .size(14.0)
                        ).selectable(false).ui(ui);

                        Label::new(
                            RichText::new(&notification.body)
                                .color(text_colour)
                                .size(10.0)
                        ).selectable(false).ui(ui);
                    });
            }
        ).response
    }
}
