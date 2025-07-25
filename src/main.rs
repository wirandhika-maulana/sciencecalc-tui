use ratatui::text::{Line, Span, Text};
use ratatui::style::{Color, Modifier};
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::Wrap;
// Fungsi utilitas warna theme sederhana
fn get_theme_color(name: &str, _theme: &str) -> Color {
    match name {
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "cyan" => Color::Cyan,
        "orange" => Color::LightRed,
        "bg" => Color::Black,
        _ => Color::White,
    }
}

fn create_ascii_header(theme: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(
                "░██████╗░█████╗░██╗███████╗███╗░░██╗░█████╗░███████╗",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░█████╗░░█████╗░██╗░░░░░░█████╗░",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "██╔════╝██╔══██╗██║██╔════╝████╗░██║██╔══██╗██╔════╝",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "██╔══██╗██╔══██╗██║░░░░░░██╔══██╗",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "╚█████╗░██║░░╚═╝██║█████╗░░██╔██╗██║██║░░╚═╝█████╗░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "██║░░╚═╝███████║██║░░░░░░██║░░╚═╝",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "░╚═══██╗██║░░██╗██║██╔══╝░░██║╚████║██║░░██╗██╔══╝░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "██║░░██╗██╔══██║██║░░░░░░██║░░██╗",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "██████╔╝╚█████╔╝██║███████╗██║░╚███║╚█████╔╝███████╗",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "╚█████╔╝██║░░██║███████╗╚█████╔╝",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "╚═════╝░░╚════╝░╚═╝╚══════╝╚═╝░░╚══╝░╚════╝░╚══════╝",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░╚════╝░╚═╝░░╚═╝╚══════╝░╚════╝░",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "                    S C I E N C E   C A L C U L A T O R                    ",
                Style::default().fg(get_theme_color("cyan", theme)).add_modifier(Modifier::BOLD),
            ),
        ]),
    ]
}

fn create_owner_line(theme: &str) -> Line<'static> {
    Line::styled(
        format!(
            " {} (2025) {} v{} ",
            "Wirandhika Maulana",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        Style::default().fg(get_theme_color("cyan", theme)).add_modifier(Modifier::BOLD),
    )
}

fn render_header(
    f: &mut Frame<'_>,
    area: Rect,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
) {
    let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().add_modifier(Modifier::BOLD))
                .bg(get_theme_color("bg", theme))
                .fg(get_theme_color("orange", theme))
                .title(" ScienceCalc Header ")
                .title_bottom(owner.clone()),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(header, area);
}
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::{event, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use std::{io, error::Error};

use log::{info, error};
use chrono::Local;

// Import library sciencecalc-rs
use sciencecalc_rs::matematika::aritmetika::{pangkat, akar_kuadrat};
use sciencecalc_rs::matematika::kombinatorika::faktorial;
// splsv dan spldv adalah fungsi static di impl Aljabar
// geometri functions do not exist as separate items, remove those imports
use sciencecalc_rs::fisika::gaya;
use sciencecalc_rs::fisika::energi;
use sciencecalc_rs::fisika::listrik;
use sciencecalc_rs::kimia::larutan;
use sciencecalc_rs::kimia::reaksi;
use sciencecalc_rs::kimia::stoikiometri;

enum Menu {
    Matematika,
    Fisika,
    Kimia,
}

enum SubMenu {
    Matematika(MatematikaSub),
    Fisika(FisikaSub),
    Kimia(KimiaSub),
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum MatematikaFunc {
    // Aritmetika
    Pangkat,
    AkarKuadrat,
    // Aljabar
    FloatToFraction,
    SPLSV,
    SPLSVFrac,
    SPLDV,
    SPLDVFrac,
    Kuadrat,
    KuadratFrac,
    Determinant2x2,
    Matriks2x2,
    Inverse2x2,
    Transpose2x2,
    Determinant3x3,
    Matriks3x3,
    Inverse3x3,
    Transpose3x3,
    // Geometri (hanya contoh, detail fungsi akan diisi di patch berikutnya)
    PersegiLuas,
    PersegiKeliling,
    PersegiPanjangLuas,
    PersegiPanjangKeliling,
    SegitigaLuas,
    SegitigaKeliling,
    LingkaranLuas,
    LingkaranKeliling,
    // Statistika
    Mean,
    Median,
    Modus,
    Varian,
    StandarDeviasi,
    // Kombinatorika
    Faktorial,
    Kombinasi,
    Permutasi,
    KombinasiPerulangan,
    PermutasiPerulangan,
    // Basis
    KonversiBasis,
    ParseNumber,
    DesimalKeBiner,
    DesimalKeOktal,
    DesimalKeHexadesimal,
    BinerKeDesimal,
    BinerKeOktal,
    BinerKeHexadesimal,
    HexadesimalKeDesimal,
    HexadesimalKeBiner,
    HexadesimalKeOktal,
    OktalKeDesimal,
    OktalKeBiner,
    OktalKeHexadesimal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum FisikaFunc {
    // Energi
    EnergiKinetik,
    EnergiPotensial,
    // Gaya
    Gaya,
    // Gerak
    GLBBPerpindahan,
    GLBBKecepatanAkhir,
    // Listrik
    OhmTegangan,
    OhmArus,
    OhmHambatan,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum KimiaFunc {
    // Gas
    TekananGasIdeal,
    // Larutan
    Molaritas,
    PHAsamKuat,
    // Reaksi
    MassaProduk,
    PersenHasil,
    // Stoikiometri
    JumlahMol,
}

enum AppState {
    Menu,
    SubMenu,
    Fungsi,
    Input,
    Output,
}

#[derive(Clone, Copy)]
enum MatematikaSub {
    Aljabar,
    Basis,
    Geometri,
    Kombinatorika,
    Statistika,
    Aritmetika,
}

#[derive(Clone, Copy)]
enum FisikaSub {
    Energi,
    Gaya,
    Gerak,
    Listrik,
}

#[derive(Clone, Copy)]
enum KimiaSub {
    Gas,
    Larutan,
    Reaksi,
    Stoikiometri,
}

struct InputField {
    label: &'static str,
    value: String,
}

struct App {
    menu: Menu,
    submenu: SubMenu,
    state: AppState,
    selected_menu: usize,
    selected_submenu: usize,
    selected_func: usize,
    input: String, // legacy, for fallback/simple input
    input_fields: Vec<InputField>,
    selected_field: usize,
    output: String,
    current_func_mtk: Option<MatematikaFunc>,
    current_func_fisika: Option<FisikaFunc>,
    current_func_kimia: Option<KimiaFunc>,
}

impl App {
    fn new() -> Self {
        Self {
            menu: Menu::Matematika,
            submenu: SubMenu::None,
            state: AppState::Menu,
            selected_menu: 0,
            selected_submenu: 0,
            selected_func: 0,
            input: String::new(),
            input_fields: vec![],
            selected_field: 0,
            output: String::new(),
            current_func_mtk: None,
            current_func_fisika: None,
            current_func_kimia: None,
        }
    }

    fn set_input_fields(&mut self) {
        self.input_fields.clear();
        self.selected_field = 0;
        // Matematika
        if let Some(func) = self.current_func_mtk {
            use MatematikaFunc::*;
            self.input_fields = match func {
                Pangkat => vec![InputField { label: "base", value: String::new() }, InputField { label: "eksponen", value: String::new() }],
                AkarKuadrat => vec![InputField { label: "x", value: String::new() }],
                Faktorial => vec![InputField { label: "n", value: String::new() }],
                Kombinasi | Permutasi | KombinasiPerulangan | PermutasiPerulangan => vec![InputField { label: "n", value: String::new() }, InputField { label: "r", value: String::new() }],
                SPLSV | SPLSVFrac => vec![InputField { label: "a", value: String::new() }, InputField { label: "b", value: String::new() }],
                SPLDV | SPLDVFrac => vec![InputField { label: "a1", value: String::new() }, InputField { label: "b1", value: String::new() }, InputField { label: "c1", value: String::new() }, InputField { label: "a2", value: String::new() }, InputField { label: "b2", value: String::new() }, InputField { label: "c2", value: String::new() }],
                Kuadrat | KuadratFrac => vec![InputField { label: "a", value: String::new() }, InputField { label: "b", value: String::new() }, InputField { label: "c", value: String::new() }],
                Determinant2x2 | Inverse2x2 => vec![InputField { label: "a", value: String::new() }, InputField { label: "b", value: String::new() }, InputField { label: "c", value: String::new() }, InputField { label: "d", value: String::new() }],
                Matriks2x2 | Transpose2x2 => vec![InputField { label: "m11", value: String::new() }, InputField { label: "m12", value: String::new() }, InputField { label: "m21", value: String::new() }, InputField { label: "m22", value: String::new() }],
                Determinant3x3 | Inverse3x3 | Transpose3x3 => vec![InputField { label: "m11", value: String::new() }, InputField { label: "m12", value: String::new() }, InputField { label: "m13", value: String::new() }, InputField { label: "m21", value: String::new() }, InputField { label: "m22", value: String::new() }, InputField { label: "m23", value: String::new() }, InputField { label: "m31", value: String::new() }, InputField { label: "m32", value: String::new() }, InputField { label: "m33", value: String::new() }],
                Matriks3x3 => vec![InputField { label: "a", value: String::new() }, InputField { label: "b", value: String::new() }],
                PersegiLuas | PersegiKeliling => vec![InputField { label: "sisi", value: String::new() }],
                PersegiPanjangLuas | PersegiPanjangKeliling => vec![InputField { label: "panjang", value: String::new() }, InputField { label: "lebar", value: String::new() }],
                SegitigaLuas => vec![InputField { label: "alas", value: String::new() }, InputField { label: "tinggi", value: String::new() }],
                SegitigaKeliling => vec![InputField { label: "sisi1", value: String::new() }, InputField { label: "sisi2", value: String::new() }, InputField { label: "sisi3", value: String::new() }],
                LingkaranLuas | LingkaranKeliling => vec![InputField { label: "r", value: String::new() }],
                Mean | Median | Modus | Varian | StandarDeviasi => vec![InputField { label: "data (pisahkan koma)", value: String::new() }],
                KonversiBasis => vec![InputField { label: "num", value: String::new() }, InputField { label: "base", value: String::new() }],
                ParseNumber => vec![InputField { label: "str", value: String::new() }, InputField { label: "base", value: String::new() }],
                DesimalKeBiner | DesimalKeOktal | DesimalKeHexadesimal => vec![InputField { label: "num", value: String::new() }],
                BinerKeDesimal | BinerKeOktal | BinerKeHexadesimal => vec![InputField { label: "str", value: String::new() }],
                HexadesimalKeDesimal | HexadesimalKeBiner | HexadesimalKeOktal => vec![InputField { label: "str", value: String::new() }],
                OktalKeDesimal | OktalKeBiner | OktalKeHexadesimal => vec![InputField { label: "str", value: String::new() }],
                _ => vec![], // <-- tambahkan ini!
            };
        }
        // Fisika
        if let Some(func) = self.current_func_fisika {
            use FisikaFunc::*;
            self.input_fields = match func {
                EnergiKinetik => vec![InputField { label: "m", value: String::new() }, InputField { label: "v", value: String::new() }],
                EnergiPotensial => vec![InputField { label: "m", value: String::new() }, InputField { label: "g", value: String::new() }, InputField { label: "h", value: String::new() }],
                Gaya => vec![InputField { label: "m", value: String::new() }, InputField { label: "a", value: String::new() }],
                GLBBPerpindahan => vec![InputField { label: "v0", value: String::new() }, InputField { label: "t", value: String::new() }, InputField { label: "a", value: String::new() }],
                GLBBKecepatanAkhir => vec![InputField { label: "v0", value: String::new() }, InputField { label: "a", value: String::new() }, InputField { label: "t", value: String::new() }],
                OhmTegangan => vec![InputField { label: "i", value: String::new() }, InputField { label: "r", value: String::new() }],
                OhmArus => vec![InputField { label: "v", value: String::new() }, InputField { label: "r", value: String::new() }],
                OhmHambatan => vec![InputField { label: "v", value: String::new() }, InputField { label: "i", value: String::new() }],
            };
        }
        // Kimia
        if let Some(func) = self.current_func_kimia {
            use KimiaFunc::*;
            self.input_fields = match func {
                TekananGasIdeal => vec![InputField { label: "n", value: String::new() }, InputField { label: "r", value: String::new() }, InputField { label: "t", value: String::new() }, InputField { label: "v", value: String::new() }],
                Molaritas => vec![InputField { label: "n", value: String::new() }, InputField { label: "V", value: String::new() }],
                PHAsamKuat => vec![InputField { label: "[H+]", value: String::new() }],
                MassaProduk => vec![InputField { label: "n", value: String::new() }, InputField { label: "Mr", value: String::new() }],
                PersenHasil => vec![InputField { label: "aktual", value: String::new() }, InputField { label: "teoritis", value: String::new() }],
                JumlahMol => vec![InputField { label: "massa", value: String::new() }, InputField { label: "Mr", value: String::new() }],
            };
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Inisialisasi logger
    env_logger::init();
    info!("[{}] ScienceCalc TUI started", Local::now().format("%Y-%m-%d %H:%M:%S"));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        error!("[{}] Error: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), err);
        println!("Error: {err}");
    }
    Ok(())
}

fn ui(f: &mut Frame<'_>, app: &App) {
    let size = f.area();
    let theme = "default";
    let ascii_lines = create_ascii_header(theme);
    let owner = create_owner_line(theme);
    let header_height = ascii_lines.len() as u16 + 4;
    let layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(1)
        .constraints([
            ratatui::layout::Constraint::Length(header_height),
            ratatui::layout::Constraint::Length(3),
            ratatui::layout::Constraint::Min(5),
        ])
        .split(size);

    render_header(f, layout[0], &ascii_lines, &owner, theme);

    // Tabs utama
    let menu_titles = ["Matematika", "Fisika", "Kimia"];
    let tabs = ratatui::widgets::Tabs::new(menu_titles.iter().map(|t| Line::from(vec![Span::raw(*t)])).collect::<Vec<Line>>())
        .select(app.selected_menu)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, layout[1]);

    match app.state {
        AppState::Menu => {
            let help = ratatui::widgets::Paragraph::new("Gunakan panah kiri/kanan untuk memilih menu utama, Enter untuk masuk submenu, q untuk keluar.")
                .block(Block::default().borders(Borders::ALL).title("Bantuan"));
            f.render_widget(help, layout[2]);
        }
        AppState::SubMenu => {
            let submenu_titles = match app.selected_menu {
                0 => ["Aljabar", "Basis", "Geometri", "Kombinatorika", "Statistika", "Aritmetika"].as_ref(),
                1 => ["Energi", "Gaya", "Gerak", "Listrik"].as_ref(),
                2 => ["Gas", "Larutan", "Reaksi", "Stoikiometri"].as_ref(),
                _ => &[],
            };
            let items: Vec<ratatui::widgets::ListItem> = submenu_titles.iter().enumerate().map(|(i, t)| {
                if i == app.selected_submenu {
                    ratatui::widgets::ListItem::new(format!("> {}", t)).style(Style::default().fg(Color::Yellow))
                } else {
                    ratatui::widgets::ListItem::new(format!("  {}", t))
                }
            }).collect();
            let submenu = ratatui::widgets::List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Submenu"));
            f.render_widget(submenu, layout[2]);
        }
        AppState::Fungsi => {
            let fungsi_titles: Vec<&str> = match app.selected_menu {
                0 => match app.selected_submenu {
                    0 => vec![ // Aljabar
                        "Float to Fraction (x)", "SPLSV (a,b)", "SPLSV Frac (a,b)", "SPLDV (a1,b1,c1,a2,b2,c2)", "SPLDV Frac (a1,b1,c1,a2,b2,c2)", "Kuadrat (a,b,c)", "Kuadrat Frac (a,b,c)", "Determinant 2x2 (a,b,c,d)", "Matriks 2x2 (m1,m2)", "Inverse 2x2 (a,b,c,d)", "Transpose 2x2 (m)", "Determinant 3x3 (m)", "Matriks 3x3 (a,b)", "Inverse 3x3 (m)", "Transpose 3x3 (m)"
                    ],
                    1 => vec![ // Basis
                        "Konversi Basis (num,base)", "Parse Number (str,base)", "Desimal ke Biner (num)", "Desimal ke Oktal (num)", "Desimal ke Hexadesimal (num)", "Biner ke Desimal (str)", "Biner ke Oktal (str)", "Biner ke Hexadesimal (str)", "Hexadesimal ke Desimal (str)", "Hexadesimal ke Biner (str)", "Hexadesimal ke Oktal (str)", "Oktal ke Desimal (str)", "Oktal ke Biner (str)", "Oktal ke Hexadesimal (str)"
                    ],
                    2 => vec![ // Geometri
                        "Persegi Luas (sisi)", "Persegi Keliling (sisi)", "Persegi Panjang Luas (panjang,lebar)", "Persegi Panjang Keliling (panjang,lebar)", "Segitiga Luas (alas,tinggi)", "Segitiga Keliling (sisi1,sisi2,sisi3)", "Lingkaran Luas (r)", "Lingkaran Keliling (r)"
                    ],
                    3 => vec![ // Kombinatorika
                        "Faktorial (n)", "Kombinasi (n,k)", "Permutasi (n,r)", "Kombinasi Perulangan (n,r)", "Permutasi Perulangan (n,[...])"
                    ],
                    4 => vec![ // Statistika
                        "Mean ([x,...])", "Median ([x,...])", "Modus ([x,...])", "Varian ([x,...])", "Standar Deviasi ([x,...])"
                    ],
                    5 => vec![ // Aritmetika
                        "Pangkat (base,eksponen)", "Akar Kuadrat (x)"
                    ],
                    _ => vec![],
                },
                1 => match app.selected_submenu {
                    0 => vec!["Energi Kinetik (m,v)", "Energi Potensial (m,g,h)"],
                    1 => vec!["Gaya (m,a)"],
                    2 => vec!["GLBB Perpindahan (v0,t,a)", "GLBB Kecepatan Akhir (v0,a,t)"],
                    3 => vec!["Ohm Tegangan (i,r)", "Ohm Arus (v,r)", "Ohm Hambatan (v,i)"],
                    _ => vec![],
                },
                2 => match app.selected_submenu {
                    0 => vec!["Tekanan Gas Ideal (n,r,t,v)"],
                    1 => vec!["Molaritas (n,V)", "pH Asam Kuat ([H+])"],
                    2 => vec!["Massa Produk (n,Mr)", "Persen Hasil (aktual,teoritis)"],
                    3 => vec!["Jumlah Mol (massa,Mr)"],
                    _ => vec![],
                },
                _ => vec![],
            };
            let items: Vec<ratatui::widgets::ListItem> = fungsi_titles.iter().enumerate().map(|(i, t)| {
                if i == app.selected_func {
                    ratatui::widgets::ListItem::new(format!("> {}", t)).style(Style::default().fg(Color::Yellow))
                } else {
                    ratatui::widgets::ListItem::new(format!("  {}", t))
                }
            }).collect();
            let fungsi = ratatui::widgets::List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Fungsi"));
            f.render_widget(fungsi, layout[2]);
        }
        AppState::Input => {
            // Contoh soal per fungsi
            let mut contoh = String::new();
            if let Some(func) = app.current_func_mtk {
                use MatematikaFunc::*;
                contoh = match func {
                    Pangkat => "Contoh: base=2, eksponen=3 (2^3=8)".to_string(),
                    AkarKuadrat => "Contoh: x=16 (akar 16=4)".to_string(),
                    Faktorial => "Contoh: n=5 (5!=120)".to_string(),
                    SPLSV => "Contoh: a=2, b=4 (2x=4, x=2)".to_string(),
                    SPLDV => "Contoh: a1=2, b1=3, c1=13, a2=1, b2=2, c2=8 (2x+3y=13, x+2y=8)".to_string(),
                    Kuadrat => "Contoh: a=1, b=-3, c=2 (x^2-3x+2=0, x=1 atau x=2)".to_string(),
                    PersegiLuas => "Contoh: sisi=4 (L=16)".to_string(),
                    PersegiPanjangLuas => "Contoh: panjang=5, lebar=3 (L=15)".to_string(),
                    SegitigaLuas => "Contoh: alas=6, tinggi=4 (L=12)".to_string(),
                    LingkaranLuas => "Contoh: r=7 (L=153.94)".to_string(),
                    Mean => "Contoh: data=1,2,3,4,5 (mean=3)".to_string(),
                    Kombinasi => "Contoh: n=5, r=2 (C(5,2)=10)".to_string(),
                    Permutasi => "Contoh: n=5, r=2 (P(5,2)=20)".to_string(),
                    KonversiBasis => "Contoh: num=10, base=2 (1010)".to_string(),
                    ParseNumber => "Contoh: str=1010, base=2 (10)".to_string(),
                    _ => "Contoh: Isi sesuai label field".to_string(),
                };
            } else if let Some(func) = app.current_func_fisika {
                use FisikaFunc::*;
                contoh = match func {
                    EnergiKinetik => "Contoh: m=2, v=3 (Ek=0.5*2*3^2=9)".to_string(),
                    EnergiPotensial => "Contoh: m=2, g=10, h=5 (Ep=2*10*5=100)".to_string(),
                    Gaya => "Contoh: m=2, a=5 (F=2*5=10)".to_string(),
                    GLBBPerpindahan => "Contoh: v0=2, t=3, a=4 (s=2*3+0.5*4*9=6+18=24)".to_string(),
                    GLBBKecepatanAkhir => "Contoh: v0=2, a=3, t=4 (vt=2+3*4=14)".to_string(),
                    OhmTegangan => "Contoh: i=2, r=5 (V=2*5=10)".to_string(),
                    OhmArus => "Contoh: v=10, r=5 (I=10/5=2)".to_string(),
                    OhmHambatan => "Contoh: v=10, i=2 (R=10/2=5)".to_string(),
                };
            } else if let Some(func) = app.current_func_kimia {
                use KimiaFunc::*;
                contoh = match func {
                    TekananGasIdeal => "Contoh: n=1, r=0.082, t=300, v=24.6 (P=1*0.082*300/24.6)".to_string(),
                    Molaritas => "Contoh: n=0.5, V=1 (M=0.5/1=0.5)".to_string(),
                    PHAsamKuat => "Contoh: [H+]=0.01 (pH=2)".to_string(),
                    MassaProduk => "Contoh: n=2, Mr=18 (m=2*18=36)".to_string(),
                    PersenHasil => "Contoh: aktual=8, teoritis=10 (80%)".to_string(),
                    JumlahMol => "Contoh: massa=10, Mr=2 (n=10/2=5)".to_string(),
                };
            }
            // Jika ada input_fields, tampilkan field per parameter
            if !app.input_fields.is_empty() {
                let mut field_widgets = vec![];
                for (i, field) in app.input_fields.iter().enumerate() {
                    let mut title = format!("{}:", field.label);
                    if i == app.selected_field {
                        title.push_str(" <");
                    }
                    let w = ratatui::widgets::Paragraph::new(field.value.as_str())
                        .block(Block::default().borders(Borders::ALL).title(title));
                    field_widgets.push(w);
                }
                // Layout field vertikal
                let constraints = vec![ratatui::layout::Constraint::Length(3); field_widgets.len()];
                let input_layout = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints(constraints)
                    .split(layout[2]);
                for (i, w) in field_widgets.iter().enumerate() {
                    f.render_widget(w.clone(), input_layout[i]);
                }
                // Tampilkan kursor di field aktif
                let x = input_layout[app.selected_field].x + app.input_fields[app.selected_field].value.len() as u16 + 1;
                let y = input_layout[app.selected_field].y + 1;
                f.set_cursor(x, y);
                // Info navigasi
                let info = ratatui::widgets::Paragraph::new("Tab/Shift+Tab untuk pindah field, Enter untuk submit, ESC untuk kembali")
                    .style(Style::default().fg(Color::DarkGray))
                    .block(Block::default().borders(Borders::NONE));
                let info_area = ratatui::layout::Rect {
                    x: input_layout.last().unwrap().x,
                    y: input_layout.last().unwrap().y + 3,
                    width: input_layout.last().unwrap().width,
                    height: 1,
                };
                f.render_widget(info, info_area);
                // Tampilkan contoh soal di bawah info navigasi
                let contoh_area = ratatui::layout::Rect {
                    x: info_area.x,
                    y: info_area.y + 1,
                    width: info_area.width,
                    height: 2,
                };
                let contoh_widget = ratatui::widgets::Paragraph::new(contoh)
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(contoh_widget, contoh_area);
            } else {
                // fallback: single input
                let input = ratatui::widgets::Paragraph::new(app.input.as_str())
                    .block(Block::default().borders(Borders::ALL).title("Input (tekan Enter untuk submit, ESC untuk kembali)"));
                f.render_widget(input, layout[2]);
                let x = layout[2].x + app.input.len() as u16 + 1;
                let y = layout[2].y + 1;
                f.set_cursor(x, y);
            }
        }
        AppState::Output => {
            let output = ratatui::widgets::Paragraph::new(app.output.as_str())
                .block(Block::default().borders(Borders::ALL).title("Hasil (Enter/ESC untuk kembali)"));
            f.render_widget(output, layout[2]);
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match app.state {
                    AppState::Menu => match key.code {
                        event::KeyCode::Enter => app.state = AppState::SubMenu,
                        event::KeyCode::Left => if app.selected_menu > 0 { app.selected_menu -= 1; },
                        event::KeyCode::Right => if app.selected_menu < 2 { app.selected_menu += 1; },
                        event::KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    AppState::SubMenu => match key.code {
                        event::KeyCode::Enter => app.state = AppState::Fungsi,
                        event::KeyCode::Up => if app.selected_submenu > 0 { app.selected_submenu -= 1; },
                        event::KeyCode::Down => {
                            let max = match app.selected_menu { 0 => 5, 1 => 3, 2 => 3, _ => 0 };
                            if app.selected_submenu < max { app.selected_submenu += 1; }
                        },
                        event::KeyCode::Esc => app.state = AppState::Menu,
                        _ => {}
                    },
                    AppState::Fungsi => match key.code {
                        event::KeyCode::Enter => {
                            app.state = AppState::Input;
                            app.input.clear();
                            app.output.clear();
                            match app.selected_menu {
                                0 => {
                                    app.current_func_mtk = Some(match app.selected_submenu {
                                        0 => match app.selected_func {
                                            0 => MatematikaFunc::FloatToFraction,
                                            1 => MatematikaFunc::SPLSV,
                                            2 => MatematikaFunc::SPLSVFrac,
                                            3 => MatematikaFunc::SPLDV,
                                            4 => MatematikaFunc::SPLDVFrac,
                                            5 => MatematikaFunc::Kuadrat,
                                            6 => MatematikaFunc::KuadratFrac,
                                            7 => MatematikaFunc::Determinant2x2,
                                            8 => MatematikaFunc::Matriks2x2,
                                            9 => MatematikaFunc::Inverse2x2,
                                            10 => MatematikaFunc::Transpose2x2,
                                            11 => MatematikaFunc::Determinant3x3,
                                            12 => MatematikaFunc::Matriks3x3,
                                            13 => MatematikaFunc::Inverse3x3,
                                            14 => MatematikaFunc::Transpose3x3,
                                            _ => MatematikaFunc::FloatToFraction
                                        },
                                        1 => match app.selected_func {
                                            0 => MatematikaFunc::KonversiBasis,
                                            1 => MatematikaFunc::ParseNumber,
                                            2 => MatematikaFunc::DesimalKeBiner,
                                            3 => MatematikaFunc::DesimalKeOktal,
                                            4 => MatematikaFunc::DesimalKeHexadesimal,
                                            5 => MatematikaFunc::BinerKeDesimal,
                                            6 => MatematikaFunc::BinerKeOktal,
                                            7 => MatematikaFunc::BinerKeHexadesimal,
                                            8 => MatematikaFunc::HexadesimalKeDesimal,
                                            9 => MatematikaFunc::HexadesimalKeBiner,
                                            10 => MatematikaFunc::HexadesimalKeOktal,
                                            11 => MatematikaFunc::OktalKeDesimal,
                                            12 => MatematikaFunc::OktalKeBiner,
                                            13 => MatematikaFunc::OktalKeHexadesimal,
                                            _ => MatematikaFunc::KonversiBasis
                                        },
                                        2 => match app.selected_func {
                                            0 => MatematikaFunc::PersegiLuas,
                                            1 => MatematikaFunc::PersegiKeliling,
                                            2 => MatematikaFunc::PersegiPanjangLuas,
                                            3 => MatematikaFunc::PersegiPanjangKeliling,
                                            4 => MatematikaFunc::SegitigaLuas,
                                            5 => MatematikaFunc::SegitigaKeliling,
                                            6 => MatematikaFunc::LingkaranLuas,
                                            7 => MatematikaFunc::LingkaranKeliling,
                                            _ => MatematikaFunc::PersegiLuas
                                        },
                                        3 => match app.selected_func {
                                            0 => MatematikaFunc::Faktorial,
                                            1 => MatematikaFunc::Kombinasi,
                                            2 => MatematikaFunc::Permutasi,
                                            3 => MatematikaFunc::KombinasiPerulangan,
                                            4 => MatematikaFunc::PermutasiPerulangan,
                                            _ => MatematikaFunc::Faktorial
                                        },
                                        4 => match app.selected_func {
                                            0 => MatematikaFunc::Mean,
                                            1 => MatematikaFunc::Median,
                                            2 => MatematikaFunc::Modus,
                                            3 => MatematikaFunc::Varian,
                                            4 => MatematikaFunc::StandarDeviasi,
                                            _ => MatematikaFunc::Mean
                                        },
                                        5 => match app.selected_func {
                                            0 => MatematikaFunc::Pangkat,
                                            1 => MatematikaFunc::AkarKuadrat,
                                            _ => MatematikaFunc::Pangkat
                                        },
                                        _ => MatematikaFunc::Pangkat
                                    });
                                }
                                1 => {
                                    app.current_func_fisika = Some(match app.selected_submenu {
                                        0 => match app.selected_func {
                                            0 => FisikaFunc::EnergiKinetik,
                                            1 => FisikaFunc::EnergiPotensial,
                                            _ => FisikaFunc::EnergiKinetik
                                        },
                                        1 => FisikaFunc::Gaya,
                                        2 => match app.selected_func {
                                            0 => FisikaFunc::GLBBPerpindahan,
                                            1 => FisikaFunc::GLBBKecepatanAkhir,
                                            _ => FisikaFunc::GLBBPerpindahan
                                        },
                                        3 => match app.selected_func {
                                            0 => FisikaFunc::OhmTegangan,
                                            1 => FisikaFunc::OhmArus,
                                            2 => FisikaFunc::OhmHambatan,
                                            _ => FisikaFunc::OhmTegangan
                                        },
                                        _ => FisikaFunc::EnergiKinetik
                                    });
                                }
                                2 => {
                                    app.current_func_kimia = Some(match app.selected_submenu {
                                        0 => KimiaFunc::TekananGasIdeal,
                                        1 => match app.selected_func {
                                            0 => KimiaFunc::Molaritas,
                                            1 => KimiaFunc::PHAsamKuat,
                                            _ => KimiaFunc::Molaritas
                                        },
                                        2 => match app.selected_func {
                                            0 => KimiaFunc::MassaProduk,
                                            1 => KimiaFunc::PersenHasil,
                                            _ => KimiaFunc::MassaProduk
                                        },
                                        3 => KimiaFunc::JumlahMol,
                                        _ => KimiaFunc::TekananGasIdeal
                                    });
                                }
                                _ => {}
                            }
                            app.set_input_fields();
                        }
                        event::KeyCode::Up => if app.selected_func > 0 { app.selected_func -= 1; },
                        event::KeyCode::Down => {
                            // Batasi agar tidak out of bounds
                            let max = match app.selected_menu {
                                0 => match app.selected_submenu {
                                    0 => 14, 1 => 13, 2 => 7, 3 => 4, 4 => 4, 5 => 1, _ => 0
                                },
                                1 => match app.selected_submenu { 0 => 1, 1 => 0, 2 => 1, 3 => 2, _ => 0 },
                                2 => match app.selected_submenu { 0 => 0, 1 => 1, 2 => 1, 3 => 0, _ => 0 },
                                _ => 0
                            };
                            if app.selected_func < max { app.selected_func += 1; }
                        },
                        event::KeyCode::Esc => app.state = AppState::SubMenu,
                        _ => {}
                    },
                    AppState::Input => {
                        match key.code {
                            event::KeyCode::Esc => app.state = AppState::Fungsi,
                            event::KeyCode::Tab => {
                                if !app.input_fields.is_empty() {
                                    app.selected_field = (app.selected_field + 1) % app.input_fields.len();
                                }
                            },
                            event::KeyCode::BackTab => {
                                if !app.input_fields.is_empty() {
                                    if app.selected_field == 0 {
                                        app.selected_field = app.input_fields.len() - 1;
                                    } else {
                                        app.selected_field -= 1;
                                    }
                                }
                            },
                            event::KeyCode::Left => {
                                if !app.input_fields.is_empty() && app.selected_field > 0 {
                                    app.selected_field -= 1;
                                }
                            },
                            event::KeyCode::Right => {
                                if !app.input_fields.is_empty() && app.selected_field < app.input_fields.len() - 1 {
                                    app.selected_field += 1;
                                }
                            },
                            event::KeyCode::Char(c) => {
                                if !app.input_fields.is_empty() {
                                    app.input_fields[app.selected_field].value.push(c);
                                } else {
                                    app.input.push(c);
                                }
                            },
                            event::KeyCode::Backspace => {
                                if !app.input_fields.is_empty() {
                                    app.input_fields[app.selected_field].value.pop();
                                } else {
                                    app.input.pop();
                                }
                            },
                            event::KeyCode::Enter => {
                                // Proses perhitungan nyata untuk beberapa fungsi utama
                                if let Some(func) = app.current_func_mtk {
                                    use MatematikaFunc::*;
                                    match func {
                                        Pangkat => {
                                            let base = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let eksponen = app.input_fields.get(1).and_then(|f| f.value.parse::<u32>().ok()).unwrap_or(0);
                                            let hasil = pangkat(base, eksponen);
                                            app.output = format!("{}^{} = {}", base, eksponen, hasil);
                                        },
                                        AkarKuadrat => {
                                            let x = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let hasil = akar_kuadrat(x);
                                            app.output = format!("√{} = {}", x, hasil);
                                        },
                                        Faktorial => {
                                            let n = app.input_fields.get(0).and_then(|f| f.value.parse::<u64>().ok()).unwrap_or(0);
                                            let hasil = faktorial(n);
                                            app.output = format!("{}! = {}", n, hasil);
                                        },
                                        SPLSV => {
                                            let a = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let b = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let x = sciencecalc_rs::matematika::aljabar::Aljabar::splsv(a, b);
                                            app.output = match x {
                                                Some(val) => format!("{}x = {}  =>  x = {}", a, b, val),
                                                None => format!("{}x = {}  =>  Tidak ada solusi (no solution)", a, b),
                                            };
                                        },
                                        SPLDV => {
                                            let a1 = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let b1 = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let c1 = app.input_fields.get(2).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let a2 = app.input_fields.get(3).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let b2 = app.input_fields.get(4).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let c2 = app.input_fields.get(5).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let (x, y) = sciencecalc_rs::matematika::aljabar::Aljabar::spldv(a1, b1, c1, a2, b2, c2).unwrap_or((0.0, 0.0));
                                            app.output = format!("Hasil: x = {}, y = {}", x, y);
                                        },
                                        PersegiLuas => {
                                            let sisi = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        },
                                        PersegiPanjangLuas => {
                                            let p = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let lbr = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        },
                                        SegitigaLuas => {
                                            let alas = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let tinggi = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        },
                                        LingkaranLuas => {
                                            let r = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        },
                                        _ => {
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        }
                                    }
                                } else if let Some(func) = app.current_func_fisika {
                                    use FisikaFunc::*;
                                    match func {
                                        Gaya => {
                                            let m = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let a = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let f = gaya::gaya(m, a);
                                            app.output = format!("F = {} N", f);
                                        },
                                        EnergiKinetik => {
                                            let m = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let v = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let ek = energi::energi_kinetik(m, v);
                                            app.output = format!("Ek = {} Joule", ek);
                                        },
                                        EnergiPotensial => {
                                            let m = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let g = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(9.8);
                                            let h = app.input_fields.get(2).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let ep = energi::energi_potensial(m, g, h);
                                            app.output = format!("Ep = {} Joule", ep);
                                        },
                                        OhmTegangan => {
                                            let i = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let r = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let v = listrik::ohm_tegangannya(i, r);
                                            app.output = format!("V = {} Volt", v);
                                        },
                                        _ => {
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        }
                                    }
                                } else if let Some(func) = app.current_func_kimia {
                                    use KimiaFunc::*;
                                    match func {
                                        Molaritas => {
                                            let n = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let v = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let m = larutan::molaritas(n, v);
                                            app.output = format!("M = {} mol/L", m);
                                        },
                                        PHAsamKuat => {
                                            let h = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let ph = larutan::ph_asam_kuat(h);
                                            app.output = format!("pH = {}", ph);
                                        },
                                        MassaProduk => {
                                            let n = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let mr = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let m = reaksi::massa_produk(n, mr);
                                            app.output = format!("m = {} gram", m);
                                        },
                                        PersenHasil => {
                                            let aktual = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let teoritis = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let persen = reaksi::persen_hasil(aktual, teoritis);
                                            app.output = format!("% hasil = {}%", persen);
                                        },
                                        JumlahMol => {
                                            let massa = app.input_fields.get(0).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(0.0);
                                            let mr = app.input_fields.get(1).and_then(|f| f.value.parse::<f64>().ok()).unwrap_or(1.0);
                                            let n = stoikiometri::jumlah_mol(massa, mr);
                                            app.output = format!("n = {} mol", n);
                                        },
                                        _ => {
                                            app.output = "[TODO] Fungsi ini belum diintegrasi ke sciencecalc-rs".to_string();
                                        }
                                    }
                                } else {
                                    app.output = "[BELUM IMPLEMENTASI] Hasil perhitungan akan muncul di sini".to_string();
                                }
                                app.state = AppState::Output;
                            },
                            _ => {}
                        }
                    },
                    AppState::Output => match key.code {
                        event::KeyCode::Esc | event::KeyCode::Enter => app.state = AppState::Fungsi,
                        _ => {}
                    },
                }
            }
        }
    }
}
