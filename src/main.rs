extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./assets/glass_original.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(& data.icon), tip: Some("Drink more water notification app"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Pause", check: false)]
    #[nwg_events(OnMenuItemSelected: [SystemTray::pause])]
    tray_item_pause: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item_exit: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn pause(&self) {
        // TODO: need to use state on struct and another thread
        if self.tray_item_pause.checked() {
            self.tray_item_pause.set_checked(false);
        } else {
            self.tray_item_pause.set_checked(true);
        }

        // TODO: this is a notification trigger
        // self.notify();
    }

    fn notify(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show("An hour has passed, you need to drink some water!",
                       Some("Drink Water Notification"),
                       Some(flags),
                       Some(&self.icon));
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    // TODO: init logging, use something simple
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}