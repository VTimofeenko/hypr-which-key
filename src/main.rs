#![allow(warnings)]
use hyprland::async_closure;
use iced::futures;
use iced::stream;
use iced::widget::{self, button, center, column, row, scrollable, text, text_input};
use iced::{color, Center, Element, Fill, Subscription, Task};

//use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::{Stream, StreamExt};

pub fn main() -> iced::Result {
    iced::application("HyprWhichKey - Iced", HyprWhichKey::update, HyprWhichKey::view)
        .theme(|_| iced::Theme::Dark)
        .subscription(HyprWhichKey::subscription)
        .run()
}

#[derive(Default)]
struct HyprWhichKey {
    submap: String,
    should_show: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EnterSubmap(String),
    ExitSubmap,
}

impl HyprWhichKey {
    fn new() -> Self {
        Self {
            submap: "".to_string(),
            should_show: false,
        }
    }

    fn update(&mut self, message: Message) {
        eprintln!("Got update: {:?}", message);
        match message {
            Message::EnterSubmap(submap) => {
                self.should_show = true;
                self.submap = submap;
            }
            Message::ExitSubmap => self.should_show = false,
        };
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(listen)
    }

    fn view(&self) -> Element<Message> {
        println!("View");
        column![text(self.submap.clone()), text("BEPIS")].into()
    }
}

pub fn listen() -> impl Stream<Item = Message> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        // Here be the event listener that connects to hyprland socket and sends the submap data
        let mut listener = hyprland::event_listener::EventListener::new();

        listener.add_sub_map_changed_handler(move |data| {
            println!("Sending {data:#?}");

            tx.send(data.to_string()).unwrap()
        });

        listener.start_listener()
    });

    stream::channel(100, move |mut output| async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            let foo = rx.recv().unwrap();
            eprintln!("{foo:#?}");
            match foo.is_empty() {
                true => output.send(Message::ExitSubmap).await,
                false => output.send(Message::EnterSubmap(foo)).await,
            };
        }
    })
}
