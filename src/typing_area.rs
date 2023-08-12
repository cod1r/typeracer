use iced::widget::text_input::{StyleSheet, Value};
use iced::{alignment, Background, BorderRadius, Color, Element, Length};
use iced_core::layout::{Layout, Limits, Node};
use iced_core::widget::Tree;
use iced_core::{mouse, renderer, Padding, Rectangle, Widget};
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
        let mut processed_txt = String::new();
        for (idx, chr) in text.char_indices() {
            if idx % 40 == 0 && idx > 0 {
                processed_txt.push('\n');
            }
            processed_txt.push(chr);
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    height: layout.bounds().height,
                    width: layout.bounds().width,
                    x: layout.position().x,
                    y: layout.position().y,
                },
                border_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
                border_radius: BorderRadius::from(0.0),
                border_width: 1.0,
            },
            Background::Color(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }),
        );
        renderer.fill_text(iced_core::Text {
            content: &processed_txt,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            bounds: iced::Rectangle {
                x: layout.position().x,
                y: layout.position().y,
                height: layout.bounds().height,
                width: layout.bounds().width,
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
        Node::with_children(
            iced_core::Size::new(500.0, 500.0),
            vec![Node::new(limits.resolve(iced::Size::ZERO))],
        )
    }
}
impl<'a, Message, Renderer> From<TypingArea> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(text_input: TypingArea) -> Element<'a, Message, Renderer> {
        Element::new(text_input)
    }
}
