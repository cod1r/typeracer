use iced::widget::text_input::{StyleSheet, Value};
use iced::{alignment, Color, Length, Element};
use iced_core::layout::{Layout, Limits, Node};
use iced_core::widget::Tree;
use iced_core::{mouse, renderer, Rectangle, Widget};
pub struct TypingArea {
    pub width: Length,
    pub height: Length,
    pub value: Value,
}

impl<Message, Renderer> Widget<Message, Renderer> for TypingArea
where
    Renderer: iced_core::Renderer + iced_core::text::Renderer,
    Renderer::Theme: iced_style::text_input::StyleSheet,
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
        let text = self.value.to_string();
        renderer.fill_text(iced_core::Text {
            content: text.as_str(),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Center,
            color: Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            bounds: iced::Rectangle {
                x: 0.0,
                y: 0.0,
                height: f32::INFINITY,
                width: f32::INFINITY,
            },
            font: renderer.default_font(),
            line_height: iced_core::text::LineHeight::default(),
            shaping: iced_core::text::Shaping::Advanced,
            size: renderer.default_size(),
        });
    }
    fn width(&self) -> Length {
        self.width
    }
    fn height(&self) -> Length {
        self.height
    }
    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        Node::with_children(iced_core::Size::INFINITY, vec![])
    }
}
impl<'a, Message, Renderer> From<TypingArea>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(
        text_input: TypingArea,
    ) -> Element<'a, Message, Renderer> {
        Element::new(text_input)
    }
}