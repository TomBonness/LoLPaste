use std::thread;

use arboard::Clipboard;
use core_foundation::runloop::CFRunLoopRun;
use enigo::{Enigo, Keyboard, Settings};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

fn main() {
    let manager =
        GlobalHotKeyManager::new().expect("Failed to create global hotkey manager");

    let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyV);
    manager
        .register(hotkey)
        .expect("Failed to register Cmd+Shift+V — may be in use by another app");

    eprintln!("LoLPaste running. Press Cmd+Shift+V to paste clipboard as keystrokes.");

    let receiver = GlobalHotKeyEvent::receiver().clone();

    thread::spawn(move || {
        let mut enigo: Option<Enigo> =
            Enigo::new(&Settings::default()).inspect_err(|e| {
                eprintln!(
                    "Accessibility permission not granted ({e}). \
                     Grant it in System Preferences → Privacy & \
                     Security → Accessibility, then press Cmd+Shift+V."
                );
            }).ok();

        while let Ok(event) = receiver.recv() {

            if event.state() != HotKeyState::Pressed {
                continue;
            }

            if enigo.is_none() {
                enigo = Enigo::new(&Settings::default()).ok();
                if enigo.is_some() {
                    eprintln!("Accessibility permission now active.");
                }
            }

            let Some(ref mut enigo) = enigo else {
                eprintln!("Still waiting for Accessibility permission…");
                continue;
            };

            let mut clipboard = match Clipboard::new() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Clipboard error: {e}");
                    continue;
                }
            };

            match clipboard.get_text() {
                Ok(text) if !text.is_empty() => {
                    if let Err(e) = enigo.text(&text) {
                        eprintln!("Keystroke simulation error: {e}");
                    }
                }
                Ok(_) => {}
                Err(e) => eprintln!("Clipboard read error: {e}"),
            }
        }
    });

    unsafe { CFRunLoopRun() };
}
