#![allow(warnings)]
use futures::sink::SinkExt;
use futures::stream::{Stream, StreamExt};
use iced::futures;
use iced::stream;
use iced::widget::{
    button, center, column, container, horizontal_space, scrollable, text, text_input,
};
use iced::window;
use iced::{Center, Element, Fill, Subscription, Task, Theme, Vector};

use std::collections::BTreeMap;

fn main() -> iced::Result {
    iced::daemon("Example", Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(Example::theme)
        .run_with(Example::new)
}

#[derive(Debug)]
struct Example {
    windowId: Option<window::Id>,
    window: Option<Window>,
}

#[derive(Debug)]
struct Window {
    submap: String,
}

#[derive(Debug, Clone)]
enum WindowMessage {
    WindowOpened(String),
    WindowClose,
}

impl Example {
    fn new() -> (Self, Task<WindowMessage>) {
        (
            Self {
                windowId: None,
                window: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        match message {
            WindowMessage::WindowOpened(submap) => {
                println!("Creating window with {submap:#?}");
                let window = Window::new(submap);

                let (id, open) = window::open(window::Settings::default());
                self.windowId = Some(id);
                self.window = Some(window);

                open.discard()
            }
            WindowMessage::WindowClose => {
                if let (Some(id), Some(window)) = (self.windowId, &self.window) {
                    window::close(id)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<WindowMessage> {
        println!("Daemon view {self:#?}");
        let window = &self.window;
        window.as_ref().unwrap().view(self.windowId.unwrap()).into()
    }

    fn theme(&self, window: window::Id) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<WindowMessage> {
        Subscription::run(listen)
    }
}

impl Window {
    fn new(submap: String) -> Self {
        Self { submap }
    }

    fn view(&self, id: window::Id) -> Element<WindowMessage> {
        let content = scrollable(
            column![text(self.submap.clone())]
                .spacing(50)
                .width(Fill)
                .align_x(Center),
        );

        container(content).center_x(200).into()
    }
}

pub fn listen() -> impl Stream<Item = WindowMessage> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut listener = hyprland::event_listener::EventListener::new();

        listener.add_sub_map_changed_handler(move |data| {
            println!("Sending {data:#?}");

            tx.send(data.to_string()).unwrap()
        });

        listener.start_listener()
    });

    stream::channel(100, move |mut output| async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(100)).await;
            let foo = rx.recv().unwrap();
            eprintln!("{foo:#?}");
            match foo.is_empty() {
                true => output.send(WindowMessage::WindowClose).await,
                false => output.send(WindowMessage::WindowOpened(foo)).await,
            };
        }
    })
}
