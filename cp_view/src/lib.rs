use druid::{widget::Label, AppLauncher, Widget, WindowDesc};

// struct State;

pub fn show() {
    let main_window = WindowDesc::new(ui_builder()).title(format!("My Complier"));

    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Launch Window Error!");
}

fn ui_builder() -> impl Widget<()> {
    Label::new(format!("you did it!"))
}
