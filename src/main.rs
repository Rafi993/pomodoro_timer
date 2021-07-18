use iced::{
    button, executor, time, window, Application, Button, Clipboard, Column, Command, Container,
    Element, Length, Row, Settings, Subscription, Text,
};

mod style;

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: (270, 200),
        always_on_top: true,
        min_size: Some((270, 200)),
        max_size: Some((270, 200)),
        resizable: false,
        transparent: false,
        ..window::Settings::default()
    };

    Pomorodo::run(Settings {
        window: window_settings,
        ..Settings::default()
    })
}

#[derive(Default)]
struct Pomorodo {
    duration: i32,
    is_running: bool,
    is_break: bool,
    timer_button: button::State,
    break_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartTimer,
    StopTimer,
    StartBreak,
    Tick,
}

impl Application for Pomorodo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Pomorodo {
                duration: 1500,
                is_break: false,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomorodo Timer")
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::StartTimer => {
                self.duration = 1500;
                self.is_running = true;
                self.is_break = false;
            }
            Message::Tick => {
                if self.is_running {
                    if !self.is_break && self.duration == 0 {
                        // Start break
                        self.duration = 300;
                        self.is_running = true;
                        self.is_break = false;
                    } else if self.is_break && self.duration == 0 {
                        // Start timer
                        self.duration = 1500;
                        self.is_running = true;
                        self.is_break = false;
                    } else {
                        // Decrement the timer
                        self.duration = self.duration - 1;
                    }
                }
            }
            Message::StopTimer => {
                self.is_running = false;
            }
            Message::StartBreak => {
                self.duration = 300;
                self.is_running = true;
                self.is_break = false;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let minutes = self.duration / 60;
        let seconds = self.duration % 60;
        let timer_text = Text::new(format!("{:02}:{:02}", minutes, seconds))
            .width(Length::Fill)
            .size(100);

        let timer_button = if self.is_running && !self.is_break {
            Button::new(&mut self.timer_button, Text::new("Stop Timer").size(20))
                .on_press(Message::StopTimer)
                .padding(10)
                .style(style::ButtonStyle)
        } else {
            Button::new(&mut self.timer_button, Text::new("Start Timer").size(20))
                .on_press(Message::StartTimer)
                .padding(10)
                .style(style::ButtonStyle)
        };

        let break_button = if self.is_running && self.is_break {
            Button::new(&mut self.break_button, Text::new("Stop Break").size(20))
                .on_press(Message::StopTimer)
                .padding(10)
                .style(style::ButtonStyle)
        } else {
            Button::new(&mut self.break_button, Text::new("Start Break").size(20))
                .on_press(Message::StartBreak)
                .padding(10)
                .style(style::ButtonStyle)
        };

        let body = Column::new()
            .max_width(800)
            .spacing(20)
            .padding(20)
            .push(timer_text)
            .push(Row::new().push(timer_button).push(break_button));

        Container::new(body)
            .width(Length::Units(270))
            .height(Length::Units(200))
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}
