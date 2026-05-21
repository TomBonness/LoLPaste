# LoLPaste

> **Cmd+Shift+V** — paste your clipboard as keystrokes into any macOS app.

A background utility that listens for a global hotkey, reads whatever text is on your clipboard, and types it out character-by-character using simulated keyboard input. Built for League of Legends in-game chat (which ignores normal paste), but works everywhere.

## How it works

```
┌──────────────────────────────┐
│  Main Thread (CFRunLoop)     │
│  - registers Cmd+Shift+V     │
│  - waits for hotkey events   │
└──────────┬───────────────────┘
           │ Carbon event
┌──────────▼───────────────────┐
│  Worker Thread               │
│  - Clipboard.get_text()      │
│  - Enigo.text(&text)         │  → CGEventPost to HID tap
└──────────────────────────────┘
```

Pressing **Cmd+Shift+V** reads the clipboard and types the text via `CGEventPost` with Unicode string support — no keycode mapping, no ASCII limitation. Emoji, accented characters, CJK — everything works.

No dock icon. No window. Just a background process.

## Prerequisites

- macOS (Apple Silicon or Intel)
- [Rust](https://rustup.rs) toolchain

## Build

```bash
git clone https://github.com/TomBonness/LoLPaste.git
cd LoLPaste
cargo build --release
```

The binary lands at `target/release/lolpaste`.

## Create the .app bundle

```bash
mkdir -p LoLPaste.app/Contents/MacOS
cp target/release/lolpaste LoLPaste.app/Contents/MacOS/
cp Info.plist LoLPaste.app/Contents/
```

The `Info.plist` sets `LSUIElement = YES` so the app stays out of your dock.

## Permissions (required)

LoLPaste simulates keyboard input via `CGEventPost`. macOS requires **Accessibility** permission for this.

1. Launch `LoLPaste.app` (double-click, or `open LoLPaste.app`).
2. macOS will block it. Open **System Preferences → Privacy & Security → Accessibility**.
3. Add `LoLPaste.app` to the list and enable the checkbox.
4. Press **Cmd+Shift+V** — if it says "Still waiting for Accessibility permission…", re-launch the app.

The app retries on every hotkey press, so you can grant permission without restarting.

## Usage

1. Copy any text (`Cmd+C`).
2. Focus the target text field — League chat, Terminal, TextEdit, browser, anything.
3. Press **Cmd+Shift+V**.
4. Text types in as if you typed it yourself.

In League: Enter to open chat → Cmd+Shift+V → Enter to send.

## Crate stack

| Concern | Crate | Mechanism |
|---------|-------|-----------|
| Global hotkey | `global-hotkey` (tauri-apps) | Carbon `RegisterEventHotKey` |
| Keystroke simulation | `enigo` | `CGEventPost` → `kCGHIDEventTap` |
| Clipboard read | `arboard` | `NSPasteboard` |

## License

MIT
