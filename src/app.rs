// src/app.rs

use crate::menu::{MenuUtama, SubMenu, SubSubMenu};
use crate::input::InputState;

/// Representasi state utama seluruh aplikasi
pub struct App {
    pub menu_utama: MenuUtama,
    pub sub_menu: Option<SubMenu>,
    pub sub_sub_menu: Option<SubSubMenu>,
    pub input_state: Option<InputState>,
    pub result: Option<String>,
    pub focus: FocusState,
    pub should_quit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusState {
    MenuUtama,
    SubMenu,
    SubSubMenu,
    Input,
    Hasil,
}

impl Default for FocusState {
    fn default() -> Self {
        FocusState::MenuUtama
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu_utama: MenuUtama::Matematika,
            sub_menu: None,
            sub_sub_menu: None,
            input_state: None,
            result: None,
            focus: FocusState::MenuUtama,
            should_quit: false,
        }
    }
}

impl App {
    /// Tangani event keyboard (navigasi menu, input, submit, dll)
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};

        match self.focus {
            FocusState::MenuUtama => {
                match key.code {
                    KeyCode::Up => self.menu_utama.prev(),
                    KeyCode::Down => self.menu_utama.next(),
                    KeyCode::Enter => {
                        self.sub_menu = Some(self.menu_utama.to_sub_menu());
                        self.focus = FocusState::SubMenu;
                    }
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => {}
                }
            }
            FocusState::SubMenu => {
                if let Some(ref mut sub) = self.sub_menu {
                    match key.code {
                        KeyCode::Up => sub.prev(),
                        KeyCode::Down => sub.next(),
                        KeyCode::Esc => {
                            self.sub_menu = None;
                            self.focus = FocusState::MenuUtama;
                        }
                        KeyCode::Enter => {
                            self.sub_sub_menu = Some(sub.to_sub_sub_menu());
                            self.focus = FocusState::SubSubMenu;
                        }
                        _ => {}
                    }
                }
            }
            FocusState::SubSubMenu => {
                if let Some(ref mut subsub) = self.sub_sub_menu {
                    match key.code {
                        KeyCode::Up => subsub.prev(),
                        KeyCode::Down => subsub.next(),
                        KeyCode::Esc => {
                            self.sub_sub_menu = None;
                            self.focus = FocusState::SubMenu;
                        }
                        KeyCode::Enter => {
                            self.input_state = Some(InputState::new(subsub));
                            self.focus = FocusState::Input;
                        }
                        _ => {}
                    }
                }
            }
            FocusState::Input => {
                if let Some(ref mut input) = self.input_state {
                    match key.code {
                        KeyCode::Tab => input.next_field(),
                        KeyCode::BackTab => input.prev_field(),
                        KeyCode::Enter => {
                            if input.all_fields_filled() {
                                self.result = Some(input.calculate_result());
                                self.focus = FocusState::Hasil;
                            }
                        }
                        KeyCode::Esc => {
                            self.input_state = None;
                            self.focus = FocusState::SubSubMenu;
                        }
                        KeyCode::Char(c) => input.input_char(c),
                        KeyCode::Backspace => input.delete_char(),
                        _ => {}
                    }
                }
            }
            FocusState::Hasil => {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        self.result = None;
                        self.input_state = None;
                        self.focus = FocusState::MenuUtama;
                        self.sub_menu = None;
                        self.sub_sub_menu = None;
                    }
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => {}
                }
            }
        }
    }
}
