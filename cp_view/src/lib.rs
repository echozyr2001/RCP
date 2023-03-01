use std::sync::Arc;

use druid::{
    commands,
    menu::{Menu, MenuItem},
    widget::{Padding, Split},
    AppDelegate, AppLauncher, Data, Env, FileDialogOptions, Lens, LocalizedString, SysMods, Widget,
    WindowDesc, WindowId,
};
use opener::open_browser;

const WINDOW_HEIGHT: f64 = 720.0;
const WINDOW_WIDTH: f64 = 1280.0;
const MIN_SIZE: f64 = 500.0;
const FONT_SIZE: f64 = 30.0;

#[derive(Data, Lens, Clone)]
pub struct AppState {
    source_text: String,
    line_number: usize,
    log_info: Arc<Vec<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            source_text: format!("this is source_text"),
            line_number: 1,
            log_info: Arc::new(Vec::new()),
        }
    }
}

mod edit_panel;
mod header;
mod info_panel;
mod menu;

pub fn show() {
    let main_window = WindowDesc::new(ui_builder())
        .menu(menu_builder)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_min_size((MIN_SIZE, MIN_SIZE))
        .title(format!("My Complier"));

    AppLauncher::with_window(main_window)
        .delegate(MyDelegate)
        .launch(AppState::new())
        .expect("Launch Window Error!");
}

fn ui_builder() -> impl Widget<AppState> {
    let header = header::build();
    let body = Split::columns(edit_panel::build(), info_panel::build());

    Padding::new(1.0, Split::rows(header, body).split_point(0.1))
}

fn menu_builder(_: Option<WindowId>, _: &AppState, _: &Env) -> Menu<AppState> {
    let base = Menu::empty();
    base.entry(
        Menu::new(LocalizedString::new("macos-menu-application-menu"))
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-about-app"))
                    // You need to handle the SHOW_ABOUT command yourself (or else do something
                    // directly to the data here instead of using a command).
                    .command(commands::SHOW_ABOUT),
            )
            .separator()
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-preferences"))
                    // You need to handle the SHOW_PREFERENCES command yourself (or else do something
                    // directly to the data here instead of using a command).
                    .command(commands::SHOW_PREFERENCES)
                    .hotkey(SysMods::Cmd, ","),
            )
            .separator()
            .entry(MenuItem::new(LocalizedString::new("macos-menu-services")))
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-hide-app"))
                    // Druid handles the HIDE_APPLICATION command automatically
                    .command(commands::HIDE_APPLICATION)
                    .hotkey(SysMods::Cmd, "h"),
            )
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-hide-others"))
                    // Druid handles the HIDE_OTHERS command automatically
                    .command(commands::HIDE_OTHERS)
                    .hotkey(SysMods::AltCmd, "h"),
            )
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-show-all"))
                    // You need to handle the SHOW_ALL command yourself (or else do something
                    // directly to the data here instead of using a command).
                    .command(commands::SHOW_ALL),
            )
            .separator()
            .entry(
                MenuItem::new(LocalizedString::new("macos-menu-quit-app"))
                    // Druid handles the QUIT_APP command automatically
                    .command(commands::QUIT_APP)
                    .hotkey(SysMods::Cmd, "q"),
            ),
    )
    .entry(
        Menu::new("File")
            .entry(MenuItem::new("Open file").on_activate(|ctx, _, _| {
                ctx.submit_command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()))
            }))
            .entry(MenuItem::new("Save as").on_activate(|ctx, _, _| {
                ctx.submit_command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()))
            })),
    )
    .entry(
        Menu::new("menu2")
            .entry(MenuItem::new("bbb"))
            .entry(MenuItem::new("ccc")),
    )
    .entry(
        Menu::new("help").entry(MenuItem::new("Open help doc").on_activate(|_, _, _| {
            open_browser("https://echo-zyr-2001s-organization.gitbook.io/rust_complier/").unwrap()
        })),
    )
}

struct MyDelegate;
impl AppDelegate<AppState> for MyDelegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.source_text = s;
                }
                Err(e) => {
                    println!("Error opening file: {e}");
                }
            }
            return druid::Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.source_text) {
                println!("Error saving file: {e}");
            }
            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}
