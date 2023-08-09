use iced::Length;
use iced_core::layout::{Layout, Limits, Node};
use iced_core::widget::Tree;
use iced_core::{mouse, renderer, Rectangle, Widget};
struct TypingArea {
    width: Length,
    height: Length,
}

impl<Message, Renderer> Widget<Message, Renderer> for TypingArea
where
    Renderer: iced_core::Renderer,
{
    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
    }
    fn width(&self) -> Length {
        self.width
    }
    fn height(&self) -> Length {
        self.height
    }
    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        todo!()
    }
}
