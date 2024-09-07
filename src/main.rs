use iced::widget::text;
use iced::{Sandbox, Settings};

fn main() -> iced::Result
{
    CubeApp::run(Settings::default())
}

struct CubeApp;

#[derive(Debug)]
enum Message {}

impl Sandbox for CubeApp
{
    type Message = Message;

    fn new() -> Self
    {
        Self 
    }

    fn title(&self) -> String 
    {
        String::from("Cube")
    }

    fn update(&mut self, message: Self::Message) 
    {
        match message 
        {

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> 
    {
        text("Testando...").into()
    }
}