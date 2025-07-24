use crate::menu::{MenuUtama, SubMenu, SubSubMenu};
use crate::input::InputState;
use crossterm::event::KeyEvent;

/// State fokus UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusState {
    MenuUtama,
    SubMenu,
    SubSubMenu,
    Input,
    Hasil,
}

pub struct App {
    pub menu_utama: MenuUtama,
    pub sub_menu: Option<SubMenu>,
    pub sub_sub_menu: Option<SubSubMenu>,
    pub input_state: Option<InputState>,
    pub result: Option<String>,
    pub focus: FocusState,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu_utama: MenuUtama::Matematika(0),
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
    /// Handler utama event keyboard
    pub fn handle_key(&mut self, key: KeyEvent) {
        use crossterm::event::KeyCode;

        match self.focus {
            FocusState::MenuUtama => {
                self.menu_utama.handle_input(key.code);
                match key.code {
                    KeyCode::Enter => {
                        self.sub_menu = Some(self.menu_utama.get_selected_sub_menu());
                        self.focus = FocusState::SubMenu;
                    }
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => {}
                }
            }
            FocusState::SubMenu => {
                if let Some(ref mut sub) = self.sub_menu {
                    sub.handle_input(key.code);
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(subsub) = sub.get_selected_sub_submenu() {
                                self.sub_sub_menu = Some(subsub);
                                self.focus = FocusState::SubSubMenu;
                            }
                        }
                        KeyCode::Esc => {
                            self.sub_menu = None;
                            self.focus = FocusState::MenuUtama;
                        }
                        _ => {}
                    }
                }
            }
            FocusState::SubSubMenu => {
                if let Some(ref mut subsub) = self.sub_sub_menu {
                    subsub.handle_input(key.code);
                    match key.code {
                        KeyCode::Enter => {
                            // Tentukan jumlah field input sesuai subsubmenu
                            let n_fields = get_field_count(subsub); // Buat fungsi ini sesuai kebutuhan
                            self.input_state = Some(InputState::with_fields(n_fields));
                            self.focus = FocusState::Input;
                        }
                        KeyCode::Esc => {
                            self.sub_sub_menu = None;
                            self.focus = FocusState::SubMenu;
                        }
                        _ => {}
                    }
                }
            }
            FocusState::Input => {
                if let Some(ref mut input) = self.input_state {
                    use crossterm::event::KeyModifiers;
                    match key.code {
                        KeyCode::Tab => input.next_field(),
                        KeyCode::BackTab => input.prev_field(),
                        KeyCode::Enter => {
                            if input.is_filled() {
                                // Kalkulasi sesuai sub_sub_menu
                                let result = do_calculation(
                                    self.sub_sub_menu.as_ref().unwrap(),
                                    &input.fields,
                                );
                                self.result = Some(result);
                                self.focus = FocusState::Hasil;
                            }
                        }
                        KeyCode::Esc => {
                            self.input_state = None;
                            self.focus = FocusState::SubSubMenu;
                        }
                        KeyCode::Char(c) => input.push_char(c),
                        KeyCode::Backspace => input.pop_char(),
                        _ => {}
                    }
                }
            }
            FocusState::Hasil => match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.result = None;
                    self.input_state = None;
                    self.focus = FocusState::MenuUtama;
                    self.sub_menu = None;
                    self.sub_sub_menu = None;
                }
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            },
        }
    }
}

/// Fungsi untuk menentukan jumlah field input per kalkulasi
fn get_field_count(subsub: &SubSubMenu) -> usize {
    use crate::menu::SubSubMenu::*;
    match subsub {
        // Contoh: SPLDV butuh 6 field (2 persamaan, 3 koefisien tiap baris)
        Aljabar(idx) => match idx {
            0 => 6, // SPLDV
            1 => 6, // SPLTV
            2 => 3, // Kuadrat
            3 => 4, // Matriks
            4 => 4, // Determinan
            5 => 4, // Invers
            _ => 2,
        },
        // Tambah kebutuhan field per subsubmenu lain sesuai kalkulasi
        _ => 2,
    }
}

/// Fungsi kalkulasi, panggil sciencecalc-rs sesuai kebutuhan
fn do_calculation(subsub: &SubSubMenu, fields: &Vec<String>) -> String {
    // Parsing input
    let nums: Vec<f64> = fields
        .iter()
        .filter_map(|s| s.trim().replace(',', ".").parse().ok())
        .collect();

    // Contoh kalkulasi SPLDV
    use crate::menu::SubSubMenu::*;
    match subsub {
        Aljabar(idx) if *idx == 0 && nums.len() == 6 => {
            // SPLDV: a1, b1, c1, a2, b2, c2
            let (a1, b1, c1, a2, b2, c2) =
                (nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
            // Panggil fungsi SPLDV dari sciencecalc
            // Misal: sciencecalc_rs::matematika::aljabar::spldv(a1, b1, c1, a2, b2, c2)
            // Dummy:
            let x = 1.0;
            let y = 1.0;
            format!("x = {}\ny = {}", x, y)
        }
        // Tambah cabang lain sesuai kebutuhan
        _ => "Perhitungan belum diimplementasikan.".to_string(),
    }
}
