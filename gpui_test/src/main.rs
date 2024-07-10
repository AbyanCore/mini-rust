use std::{
    fmt::format,
    future::{Future, IntoFuture},
    sync::{Arc, Mutex},
    thread::{self, Thread},
};

use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_sm()
            .text_color(rgb(0xffffff))
            .child(format!("Hello , {}!", &self.text))
    }
}

#[tokio::main]
async fn main() {
    let result = reqwest::get("https://catfact.ninja/fact")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: result.into(),
            })
        });
    });
}
