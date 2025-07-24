use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuUtama {
    Matematika(usize),
    Fisika(usize),
    Kimia(usize),
}

impl Default for MenuUtama {
    fn default() -> Self {
        MenuUtama::Matematika(0)
    }
}

impl MenuUtama {
    pub fn selected(&self) -> usize {
        match self {
            MenuUtama::Matematika(_) => 0,
            MenuUtama::Fisika(_) => 1,
            MenuUtama::Kimia(_) => 2,
        }
    }
    pub fn handle_input(&mut self, key: KeyCode) {
        let idx = self.selected();
        match key {
            KeyCode::Up => {
                let new_idx = if idx == 0 { 2 } else { idx - 1 };
                *self = match new_idx {
                    0 => MenuUtama::Matematika(0),
                    1 => MenuUtama::Fisika(0),
                    2 => MenuUtama::Kimia(0),
                    _ => MenuUtama::Matematika(0),
                }
            }
            KeyCode::Down => {
                let new_idx = if idx == 2 { 0 } else { idx + 1 };
                *self = match new_idx {
                    0 => MenuUtama::Matematika(0),
                    1 => MenuUtama::Fisika(0),
                    2 => MenuUtama::Kimia(0),
                    _ => MenuUtama::Matematika(0),
                }
            }
            _ => {}
        }
    }
    pub fn get_selected_sub_menu(&self) -> SubMenu {
        match self {
            MenuUtama::Matematika(_) => SubMenu::Matematika(0),
            MenuUtama::Fisika(_) => SubMenu::Fisika(0),
            MenuUtama::Kimia(_) => SubMenu::Kimia(0),
        }
    }
}

/// SUBMENU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubMenu {
    Matematika(usize),
    Fisika(usize),
    Kimia(usize),
}

impl SubMenu {
    pub fn handle_input(&mut self, key: KeyCode) {
        let (max, idx) = match self {
            SubMenu::Matematika(i) => (6, *i),
            SubMenu::Fisika(i) => (4, *i),
            SubMenu::Kimia(i) => (4, *i),
        };
        let new_idx = match key {
            KeyCode::Up => if idx == 0 { max - 1 } else { idx - 1 },
            KeyCode::Down => if idx + 1 >= max { 0 } else { idx + 1 },
            _ => idx,
        };
        match self {
            SubMenu::Matematika(_) => *self = SubMenu::Matematika(new_idx),
            SubMenu::Fisika(_) => *self = SubMenu::Fisika(new_idx),
            SubMenu::Kimia(_) => *self = SubMenu::Kimia(new_idx),
        }
    }

    pub fn get_selected_sub_submenu(&self) -> Option<SubSubMenu> {
        match self {
            SubMenu::Matematika(i) => match i {
                0 => Some(SubSubMenu::Aljabar(0)),
                1 => Some(SubSubMenu::Basis(0)),
                2 => Some(SubSubMenu::Aritmetika(0)),
                3 => Some(SubSubMenu::Geometri(0)),
                4 => Some(SubSubMenu::Kombinatorika(0)),
                5 => Some(SubSubMenu::Statistika(0)),
                _ => None,
            },
            SubMenu::Fisika(i) => match i {
                0 => Some(SubSubMenu::Energi(0)),
                1 => Some(SubSubMenu::Gaya(0)),
                2 => Some(SubSubMenu::Listrik(0)),
                3 => Some(SubSubMenu::Gerak(0)),
                _ => None,
            },
            SubMenu::Kimia(i) => match i {
                0 => Some(SubSubMenu::Gas(0)),
                1 => Some(SubSubMenu::Larutan(0)),
                2 => Some(SubSubMenu::Reaksi(0)),
                3 => Some(SubSubMenu::Stoikiometri(0)),
                _ => None,
            },
        }
    }

    pub fn render_info(&self) -> (&'static str, Vec<&'static str>, usize) {
        match self {
            SubMenu::Matematika(i) => (
                "Matematika",
                vec![
                    "Aljabar",
                    "Basis",
                    "Aritmetika",
                    "Geometri",
                    "Kombinatorika",
                    "Statistika",
                ],
                *i,
            ),
            SubMenu::Fisika(i) => (
                "Fisika",
                vec!["Energi", "Gaya", "Listrik", "Gerak"],
                *i,
            ),
            SubMenu::Kimia(i) => (
                "Kimia",
                vec!["Gas", "Larutan", "Reaksi", "Stoikiometri"],
                *i,
            ),
        }
    }
}

/// SUB-SUBMENU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubSubMenu {
    // Matematika
    Aljabar(usize),
    Basis(usize),
    Aritmetika(usize),
    Geometri(usize),
    Kombinatorika(usize),
    Statistika(usize),
    // Fisika
    Energi(usize),
    Gaya(usize),
    Listrik(usize),
    Gerak(usize),
    // Kimia
    Gas(usize),
    Larutan(usize),
    Reaksi(usize),
    Stoikiometri(usize),
}

impl SubSubMenu {
    pub fn handle_input(&mut self, key: KeyCode) {
        use SubSubMenu::*;
        let (max, idx, variant) = match self {
            // Matematika
            Aljabar(i)      => (5, *i, 0), // 5 sub
            Basis(i)        => (4, *i, 1),
            Aritmetika(i)   => (6, *i, 2),
            Geometri(i)     => (6, *i, 3),
            Kombinatorika(i)=> (2, *i, 4),
            Statistika(i)   => (4, *i, 5),
            // Fisika
            Energi(i)       => (2, *i, 6),
            Gaya(i)         => (2, *i, 7),
            Listrik(i)      => (2, *i, 8),
            Gerak(i)        => (4, *i, 9),
            // Kimia
            Gas(i)          => (2, *i, 10),
            Larutan(i)      => (3, *i, 11),
            Reaksi(i)       => (2, *i, 12),
            Stoikiometri(i) => (3, *i, 13),
        };
        let new_idx = match key {
            KeyCode::Up => if idx == 0 { max - 1 } else { idx - 1 },
            KeyCode::Down => if idx + 1 >= max { 0 } else { idx + 1 },
            _ => idx,
        };

        *self = match variant {
            // Matematika
            0 => Aljabar(new_idx),
            1 => Basis(new_idx),
            2 => Aritmetika(new_idx),
            3 => Geometri(new_idx),
            4 => Kombinatorika(new_idx),
            5 => Statistika(new_idx),
            // Fisika
            6 => Energi(new_idx),
            7 => Gaya(new_idx),
            8 => Listrik(new_idx),
            9 => Gerak(new_idx),
            // Kimia
            10 => Gas(new_idx),
            11 => Larutan(new_idx),
            12 => Reaksi(new_idx),
            13 => Stoikiometri(new_idx),
            _ => *self,
        }
    }

    pub fn render_info(&self) -> (&'static str, Vec<&'static str>, usize) {
        use SubSubMenu::*;
        match self {
            // Matematika
            Aljabar(i) => (
                "Aljabar",
                vec![
                    "Sistem Persamaan Linear",
                    "Persamaan Kuadrat",
                    "Matriks",
                    "Determinan",
                    "Invers Matriks",
                ],
                *i,
            ),
            Basis(i) => (
                "Basis",
                vec![
                    "Konversi Biner",
                    "Konversi Oktal",
                    "Konversi Desimal",
                    "Konversi Heksadesimal",
                ],
                *i,
            ),
            Aritmetika(i) => (
                "Aritmetika",
                vec![
                    "Penjumlahan",
                    "Pengurangan",
                    "Perkalian",
                    "Pembagian",
                    "Pangkat",
                    "Akar",
                ],
                *i,
            ),
            Geometri(i) => (
                "Geometri",
                vec![
                    "Luas Persegi",
                    "Luas Segitiga",
                    "Luas Lingkaran",
                    "Volume Kubus",
                    "Volume Balok",
                    "Volume Tabung",
                ],
                *i,
            ),
            Kombinatorika(i) => (
                "Kombinatorika",
                vec!["Permutasi", "Kombinasi"],
                *i,
            ),
            Statistika(i) => (
                "Statistika",
                vec![
                    "Mean",
                    "Median",
                    "Modus",
                    "Standar Deviasi",
                ],
                *i,
            ),
            // Fisika
            Energi(i) => (
                "Energi",
                vec!["Energi Kinetik", "Energi Potensial"],
                *i,
            ),
            Gaya(i) => (
                "Gaya",
                vec!["Gaya Berat", "Gaya Gesek"],
                *i,
            ),
            Listrik(i) => (
                "Listrik",
                vec!["Hukum Ohm", "Daya Listrik"],
                *i,
            ),
            Gerak(i) => (
                "Gerak",
                vec!["GLB", "GLBB", "Jarak", "Kecepatan"],
                *i,
            ),
            // Kimia
            Gas(i) => (
                "Gas",
                vec!["Hukum Boyle", "Hukum Charles"],
                *i,
            ),
            Larutan(i) => (
                "Larutan",
                vec!["Molaritas", "Molalitas", "Fraksi Mol"],
                *i,
            ),
            Reaksi(i) => (
                "Reaksi",
                vec!["Reaksi Redoks", "Kesetimbangan"],
                *i,
            ),
            Stoikiometri(i) => (
                "Stoikiometri",
                vec!["Perhitungan Mol", "Massa", "Volume Gas"],
                *i,
            ),
        }
    }
}
