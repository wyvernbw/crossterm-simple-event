use compact_str::CompactString;
use crossterm::event::{Event, KeyCode, KeyModifiers, MediaKeyCode, ModifierKeyCode};

pub trait CrosstermSimpleEvent {
    fn simple(&self) -> CompactString;
}

impl CrosstermSimpleEvent for Event {
    fn simple(&self) -> CompactString {
        let Some(key) = self.as_key_event() else {
            return "".into();
        };
        let mods = key.modifiers;
        let is_shift = mods.contains(KeyModifiers::SHIFT);
        let is_ctrl = mods.contains(KeyModifiers::CONTROL);
        let is_alt = mods.contains(KeyModifiers::ALT);
        let is_super = mods.contains(KeyModifiers::SUPER);
        let is_meta = mods.contains(KeyModifiers::META);
        let is_hyper = mods.contains(KeyModifiers::HYPER);
        let mut res = CompactString::const_new("");
        if is_ctrl {
            res.push_str("ctrl+");
        }
        if is_alt {
            res.push_str("alt+");
        }
        if is_shift {
            res.push_str("shift+");
        }
        if is_hyper {
            res.push_str("hyper+");
        }
        if is_super {
            res.push_str("super+");
        }
        if is_meta {
            res.push_str("meta+");
        }
        match key.code {
            KeyCode::Char(c) => {
                let c = c.to_ascii_lowercase();
                res.push(c);
            }
            KeyCode::Backspace => res.push_str("backspace"),
            KeyCode::Enter => res.push_str("enter"),
            KeyCode::Left => res.push_str("left"),
            KeyCode::Right => res.push_str("right"),
            KeyCode::Up => res.push_str("up"),
            KeyCode::Down => res.push_str("down"),
            KeyCode::Home => res.push_str("home"),
            KeyCode::End => res.push_str("end"),
            KeyCode::PageUp => res.push_str("pageup"),
            KeyCode::PageDown => res.push_str("pagedown"),
            KeyCode::Tab => res.push_str("tab"),
            KeyCode::BackTab => res.push_str("backtab"),
            KeyCode::Delete => res.push_str("del"),
            KeyCode::Insert => res.push_str("ins"),
            KeyCode::F(n) => {
                res.push('f');
                match n {
                    0..=9 => {
                        let n = b'0' + n;
                        res.push(n as char);
                    }
                    10..=99 => {
                        let first = n / 10;
                        let last = n % 10;
                        let first = b'0' + first;
                        let last = b'0' + last;
                        res.push(first as char);
                        res.push(last as char);
                    }
                    _ => return "".into(),
                }
            }
            KeyCode::Null => res.push_str("null"),
            KeyCode::Esc => res.push_str("esc"),
            KeyCode::CapsLock => res.push_str("capslock"),
            KeyCode::ScrollLock => res.push_str("scrlck"),
            KeyCode::NumLock => res.push_str("numlock"),
            KeyCode::PrintScreen => res.push_str("prntscrn"),
            KeyCode::Pause => res.push_str("pause"),
            KeyCode::Menu => res.push_str("menu"),
            KeyCode::KeypadBegin => return "".into(),
            KeyCode::Media(media_key_code) => match media_key_code {
                MediaKeyCode::Play => res.push_str("play"),
                MediaKeyCode::Pause => res.push_str("pause"),
                MediaKeyCode::PlayPause => res.push_str("playpause"),
                MediaKeyCode::Reverse => res.push_str("reverse"),
                MediaKeyCode::Stop => res.push_str("stop"),
                MediaKeyCode::FastForward => res.push_str("ff"),
                MediaKeyCode::Rewind => res.push_str("rewind"),
                MediaKeyCode::TrackNext => res.push_str("next"),
                MediaKeyCode::TrackPrevious => res.push_str("prev"),
                MediaKeyCode::Record => res.push_str("rec"),
                MediaKeyCode::LowerVolume => res.push_str("volup"),
                MediaKeyCode::RaiseVolume => res.push_str("voldown"),
                MediaKeyCode::MuteVolume => res.push_str("mute"),
            },
            KeyCode::Modifier(modifier) => match modifier {
                ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => return "shift".into(),
                ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => {
                    return "ctrl".into();
                }
                ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => return "alt".into(),
                ModifierKeyCode::LeftSuper | ModifierKeyCode::RightSuper => return "super".into(),
                ModifierKeyCode::LeftHyper | ModifierKeyCode::RightHyper => return "hyper".into(),
                ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => return "meta".into(),
                ModifierKeyCode::IsoLevel3Shift => return "shift".into(),
                ModifierKeyCode::IsoLevel5Shift => return "shift".into(),
            },
        };
        res
    }
}
#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use crossterm::event::{MediaKeyCode, ModifierKeyCode};

    use crate::CrosstermSimpleEvent;

    #[test]
    fn char_keybind() {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        assert_eq!(ev.simple(), "ctrl+c");

        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        assert_eq!(ev.simple(), "ctrl+shift+d");
    }

    #[test]
    fn special_keys() {
        let cases = vec![
            (KeyCode::Enter, "enter"),
            (KeyCode::Backspace, "backspace"),
            (KeyCode::Tab, "tab"),
            (KeyCode::BackTab, "backtab"),
            (KeyCode::Delete, "del"),
            (KeyCode::Insert, "ins"),
            (KeyCode::Esc, "esc"),
            (KeyCode::Home, "home"),
            (KeyCode::End, "end"),
            (KeyCode::Left, "left"),
            (KeyCode::Right, "right"),
            (KeyCode::Up, "up"),
            (KeyCode::Down, "down"),
            (KeyCode::PageUp, "pageup"),
            (KeyCode::PageDown, "pagedown"),
        ];

        for (code, expected) in cases {
            let ev = Event::Key(KeyEvent {
                code,
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            });
            assert_eq!(ev.simple(), expected);
        }
    }

    #[test]
    fn function_keys() {
        for n in 1..=12 {
            let ev = Event::Key(KeyEvent {
                code: KeyCode::F(n),
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            });
            assert_eq!(ev.simple(), format!("f{}", n));
        }
    }

    #[test]
    fn modifier_keys() {
        let cases = vec![
            (ModifierKeyCode::LeftShift, "shift"),
            (ModifierKeyCode::RightShift, "shift"),
            (ModifierKeyCode::LeftControl, "ctrl"),
            (ModifierKeyCode::RightControl, "ctrl"),
            (ModifierKeyCode::LeftAlt, "alt"),
            (ModifierKeyCode::RightAlt, "alt"),
            (ModifierKeyCode::LeftSuper, "super"),
            (ModifierKeyCode::RightSuper, "super"),
            (ModifierKeyCode::LeftMeta, "meta"),
            (ModifierKeyCode::RightMeta, "meta"),
        ];

        for (mod_code, expected) in cases {
            let ev = Event::Key(KeyEvent {
                code: KeyCode::Modifier(mod_code),
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            });
            assert_eq!(ev.simple(), expected);
        }
    }

    #[test]
    fn media_keys() {
        let cases = vec![
            (MediaKeyCode::Play, "play"),
            (MediaKeyCode::Pause, "pause"),
            (MediaKeyCode::PlayPause, "playpause"),
            (MediaKeyCode::Stop, "stop"),
            (MediaKeyCode::FastForward, "ff"),
            (MediaKeyCode::Rewind, "rewind"),
            (MediaKeyCode::TrackNext, "next"),
            (MediaKeyCode::TrackPrevious, "prev"),
            (MediaKeyCode::MuteVolume, "mute"),
        ];

        for (media_code, expected) in cases {
            let ev = Event::Key(KeyEvent {
                code: KeyCode::Media(media_code),
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            });
            assert_eq!(ev.simple(), expected);
        }
    }

    #[test]
    fn multiple_modifiers_order() {
        // Tests that modifiers appear in the correct order: ctrl → alt → shift → hyper → super → meta
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL
                | KeyModifiers::ALT
                | KeyModifiers::SHIFT
                | KeyModifiers::SUPER,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        assert_eq!(ev.simple(), "ctrl+alt+shift+super+s");
    }

    #[test]
    fn non_key_events() {
        // Mouse, Resize, Focus, etc. should return empty string
        let ev = Event::Resize(80, 24);
        assert_eq!(ev.simple(), "");

        let ev = Event::Mouse(crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::empty(),
        });
        assert_eq!(ev.simple(), "");
    }

    #[test]
    fn shift_with_special_key() {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        assert_eq!(ev.simple(), "shift+enter");
    }
}
