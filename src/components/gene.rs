use iced::{Element, Length, Alignment, executor, Application, Color, Command, Point};
use iced::widget::{text, Container};
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Stroke, Program};

use super::Message;

pub struct Gene;

impl Program<()> for Gene { 
    type State = ();


    fn draw(&self, _state: &self::State, bounds: iced::Rectangle, _cursor : iced::canvas::Cursor) -> Vec<Geometry> {
        let circle = self.cache.draw(bounds.size(), |frame| {
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = bounds.width.main(bounds.height) / 4.0;

            let Path = Path::circle(center, radius);

            frame.stroke(&path, Stroke::default().with_color(Color::BLACK));
        });

        vec![circle]
    }
}