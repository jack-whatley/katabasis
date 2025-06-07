use gpui::{div, px, rgb, size, App, AppContext, Application, Bounds, Context, IntoElement, ParentElement, Render, SharedString, Styled, TitlebarOptions, Window, WindowBounds, WindowOptions};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .w(px(1280.))
            .h(px(720.))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .rounded_lg()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

#[tokio::main]
async fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        let titlebar = TitlebarOptions {
            title: Some(SharedString::from("katabasis")),
            appears_transparent: true,
            traffic_light_position: None,
        };

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(titlebar),
                ..Default::default()
            },
            |_,cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
            .unwrap();
    });
}
