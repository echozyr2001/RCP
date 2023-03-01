// #[allow(unused_assignments)]
// fn make_menu(_: Option<WindowId>, _state: &State, _: &Env) -> Menu<State> {
//     let mut base = Menu::empty();
//     #[cfg(target_os = "macos")]
//     {
//         base = druid::platform_menus::mac::menu_bar();
//     }
//     #[cfg(any(
//         target_os = "windows",
//         target_os = "freebsd",
//         target_os = "linux",
//         target_os = "openbsd"
//     ))]
//     {
//         base = base.entry(druid::platform_menus::win::file::default());
//     }
//     let mut custom = Menu::new(LocalizedString::new("Custom"));
//     let custom2 = Menu::new(LocalizedString::new("Custom2"));
//
//     custom = custom.entry(
//         MenuItem::new(
//             LocalizedString::new("hello-counter"), // .with_arg("count", move |_: &State, _| i.into()),
//         )
//         .on_activate(move |_, _, _| println!("dsd")), // .enabled_if(move |_data, _env| i % 3 != 0)
//                                                       // .selected_if(move |data, _env| i == data.selected),
//     );
//     base = base.entry(custom);
//     base = base.entry(custom2);
//     base
//     // base.rebuild_on(|old_data, data, _env| old_data.menu_count != data.menu_count)
// }
