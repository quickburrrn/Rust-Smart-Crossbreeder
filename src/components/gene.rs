use iced::{Element, Length, Alignment, executor, Application, Color, Command, Point, Rectangle, Renderer, Theme, Size};
use iced::alignment;
use iced::widget::{text, Container, canvas, container};
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Stroke, Program, Cache, stroke};
use iced::mouse;


use super::Message;

pub struct Gene;

impl Program<()> for Gene { 
    type State = ();


    fn draw(&self,_state: &Self::State, renderer: &Renderer,theme: &Theme,bounds: iced::Rectangle, _cursor : mouse::Cursor) ->  Vec<Geometry>{
        let mut frame = Frame::new(renderer, bounds.size());

        frame.stroke(
            &Path::rectangle(
                Point {
                    x: bounds.width / 10 as f32,
                    y : bounds.height / 10 as f32,
                },
                Size {
                    width: 4. * bounds.width / 5.,
                    height: 4. * bounds.height / 5.,
                },
            ),
            Stroke::default()
        );
        vec![frame.into_geometry()]
    // let circle : Vec<Geometry> = self.draw(renderer, bounds.size(), |frame| {
    //     let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
    //     let radius = bounds.width.min(bounds.height) / 4.0;

    //     let Path = Path::circle(center, radius);

    //     frame.stroke(&Path, Stroke::default().with_color(Color::BLACK));
    // });

    // vec![circle]
    }
}