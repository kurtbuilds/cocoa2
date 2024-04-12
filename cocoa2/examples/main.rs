use cocoa2::base::selector;
use cocoa2::NSRect;
use cocoa2::NSApplication;
use cocoa2::NSAutoreleasePool;
use cocoa2::NSMenu;
use cocoa2::NSMenuItem;
use cocoa2::NSProcessInfo;
use cocoa2::{NSApplicationActivationOptions, NSRunningApplication};
use cocoa2::NSString;
use cocoa2::{NSBackingStoreType, NSWindow, StyleMask};

use std::ops::Deref;

fn main() {
    let _pool = NSAutoreleasePool::new();
    let app = NSApplication::sharedApplication();
    app.setActivationPolicy(cocoa2::ActivationPolicy::Regular);
    add_menu(&app);
    add_window();
    focus_app();
    app.run();
}

fn add_menu(app: &NSApplication) {
    // create menu bar
    let main_menu = NSMenu::newWithTitle(&NSString::from(""));
    let menubar_item = NSMenuItem::new();
    main_menu.addItem(&menubar_item);
    app.setMainMenu(&main_menu);

    // create application menu
    let app_menu = NSMenu::newWithTitle(&NSString::from("Application"));
    let process_name = NSProcessInfo::processInfo().processName();
    let quit_action = selector("terminate:");
    let title = format!("Quit {}", process_name.deref());
    let quit_item = NSMenuItem::newWith(&title, quit_action, "q");
    app_menu.addItem(&quit_item);
    menubar_item.setSubmenu(&app_menu);
}

fn add_window() {
    let rect = NSRect::new(0., 0., 200., 200.);
    let window = NSWindow::newWith(rect, StyleMask::Titled | StyleMask::Resizable | StyleMask::Closable | StyleMask::Miniaturizable, NSBackingStoreType::Buffered, false);
    window.center();
    let title = NSString::from("Hello, world!");
    window.setTitle(&title);
    window.makeKeyAndOrderFront();
}

fn focus_app() {
    NSRunningApplication::currentApplication().activateWithOptions(NSApplicationActivationOptions::IgnoringOtherApps);
}