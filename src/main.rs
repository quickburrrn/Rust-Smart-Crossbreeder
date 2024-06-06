mod components;

use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced::widget::{button, column, text, Canvas, Column};
use components::gene::{self, Gene};

// Define the main struct for the application
struct MyApp;

#[derive(Debug, Clone, Copy)]
pub enum Message{

}


impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (MyApp, Command<Self::Message>){
        (MyApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("This is the title")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        
        // Column::new()
        //     .push(text("Hello World").size(60))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .align_items(iced::Alignment::Center)
        //     .push(text("Hello").size(40))
        //     .push(button("Hello"))
        //     .into()

        Canvas::new(Gene)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

}

fn main() -> iced::Result{
    // Run the application with the default settings
    MyApp::run(Settings::default())
}