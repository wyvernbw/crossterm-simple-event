mod keybinds;

use crossterm::event::{Event, KeyModifiers, ModifierKeyCode};

use crate::keybinds::*;

pub trait CrosstermSimpleEvent {
    fn simple(&self) -> &'static str;
}

const fn key_index(
    is_shift: bool,
    is_ctrl: bool,
    is_alt: bool,
    is_super: bool,
    is_hyper: bool,
    is_meta: bool,
    offset: usize,
) -> usize {
    let mut idx = 0x0u16;
    if is_shift {
        idx |= SHIFT;
    }
    if is_ctrl {
        idx |= CTRL;
    }
    if is_alt {
        idx |= ALT;
    }
    if is_hyper {
        idx |= HYPER;
    }
    if is_meta {
        idx |= META;
    }
    if is_super {
        idx |= META
    }
    let idx = idx as usize;

    idx | offset
}

const fn modifiers_to_idx(mods: KeyModifiers) -> usize {
    let is_shift = mods.contains(KeyModifiers::SHIFT);
    let is_ctrl = mods.contains(KeyModifiers::CONTROL);
    let is_alt = mods.contains(KeyModifiers::ALT);
    let is_super = mods.contains(KeyModifiers::SUPER);
    let is_meta = mods.contains(KeyModifiers::META);
    let is_hyper = mods.contains(KeyModifiers::HYPER);
    key_index(is_shift, is_ctrl, is_alt, is_super, is_hyper, is_meta, 0)
}

impl CrosstermSimpleEvent for Event {
    fn simple(&self) -> &'static str {
        let Some(key) = self.as_key_event() else {
            return "";
        };
        let base = modifiers_to_idx(key.modifiers);
        match key.code {
            crossterm::event::KeyCode::Char(c) => {
                let offset = letter_idx(c);
                let idx = base | offset;
                KEYBINDS[idx]
            }
            crossterm::event::KeyCode::Backspace => todo!(),
            crossterm::event::KeyCode::Enter => todo!(),
            crossterm::event::KeyCode::Left => todo!(),
            crossterm::event::KeyCode::Right => todo!(),
            crossterm::event::KeyCode::Up => todo!(),
            crossterm::event::KeyCode::Down => todo!(),
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => todo!(),
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(modifier) => match modifier {
                ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => "shift",
                ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => "ctrl",
                ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => "alt",
                ModifierKeyCode::LeftSuper | ModifierKeyCode::RightSuper => "super",
                ModifierKeyCode::LeftHyper | ModifierKeyCode::RightHyper => "hyper",
                ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => "meta",
                ModifierKeyCode::IsoLevel3Shift => "shift",
                ModifierKeyCode::IsoLevel5Shift => "shift",
            },
        }
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
