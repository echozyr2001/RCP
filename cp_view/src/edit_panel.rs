use druid::{
    widget::{Controller, TextBox},
    Code, Event, Widget, WidgetExt,
};

use crate::{AppState, FONT_SIZE};

pub fn build() -> impl Widget<AppState> {
    TextBox::multiline()
        .with_text_size(FONT_SIZE)
        .with_line_wrapping(false)
        .lens(AppState::source_text)
        .controller(MyController)
}

struct MyController;
impl<W: Widget<AppState>> Controller<AppState, W> for MyController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        match event {
            Event::KeyDown(key_event) => {
                if key_event.code == Code::Enter || key_event.code == Code::Backspace {
                    let line_number = data.source_text.split('\n').count();
                    data.line_number = line_number;
                    child.event(ctx, event, data, env);
                }
            }
            Event::KeyUp(key_event) => {
                if key_event.code == Code::Enter || key_event.code == Code::Backspace {
                    let line_number = data.source_text.split('\n').count();
                    data.line_number = line_number;
                    child.event(ctx, event, data, env);
                }
            }
            // TODO: When open file then line_number will not change
            // Event::ImeStateChange => {
            //     let line_number = data.source_text.split('\n').count();
            //     data.line_number = line_number;
            //     child.event(ctx, event, data, env);
            // }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}
