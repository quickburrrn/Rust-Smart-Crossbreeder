use iced::{Application, Settings, Element, Command, executor, Theme};
use iced::widget::{text, Column, column};

// Define the main struct for the application
struct MyApp;

impl Application for MyApp {
    // Specify the executor type for asynchronous actions
    type Executor = executor::Default;
    
    // Define the type of messages that the application will handle
    type Message = ();
    
    // Define the type of flags the application accepts at startup
    type Flags = ();

    type Theme = Theme;

    // Initialize the application
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Create a new instance of MyApp and return it with no commands
        (MyApp, Command::none())
    }

    // Set the title of the application window
    fn title(&self) -> String {
        String::from("Iced Application")
        
    }

    // Update the application state based on messages
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        // Since this application does nothing, just return no commands
        Command::none()
    }

    // Define how the application's UI looks
    // fn view(&self) -> Element<Self::Message> {
    //     // Create an empty element (a blank text widget in this case)
    //     Element::new(text("hello world"))
    // }

    fn view(&self) -> Element<Self::Message> {
        Element::new(iced::widget::Text::new(""))
    }
}

fn main() {
    // Run the application with the default settings
    MyApp::run(Settings::default());
}