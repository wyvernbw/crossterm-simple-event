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
                let n = b'0' + n;
                res.push(n as char);
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
        assert_eq!(ev.simple(), "ctrl+shift+d")
    }
}
