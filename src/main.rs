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

fn render_header<B: Backend>(
    f: &mut Frame<B>,
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
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, ListState};
use crossterm::{event, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use std::{io, error::Error};
use clap::Parser;
use log::{info, error};
use chrono::Local;

// Import library sciencecalc-rs
use sciencecalc_rs::{
    matematika, fisika, kimia
};

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

#[derive(Clone, Copy, PartialEq)]
#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

struct App {
    menu: Menu,
    submenu: SubMenu,
    state: AppState,
    selected_menu: usize,
    selected_submenu: usize,
    selected_func: usize,
    input: String,
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
            output: String::new(),
            current_func_mtk: None,
            current_func_fisika: None,
            current_func_kimia: None,
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match app.state {
                    AppState::Menu => match key.code {
                        event::KeyCode::Char('q') => return Ok(()),
                        event::KeyCode::Left => {
                            if app.selected_menu == 0 {
                                app.selected_menu = 2;
                            } else {
                                app.selected_menu -= 1;
                            }
                        }
                        event::KeyCode::Right => {
                            app.selected_menu = (app.selected_menu + 1) % 3;
                        }
                        event::KeyCode::Enter => {
                            app.state = AppState::SubMenu;
                            app.selected_submenu = 0;
                        }
                        _ => {}
                    },
                    AppState::SubMenu => match key.code {
                        event::KeyCode::Esc => {
                            app.state = AppState::Menu;
                        }
                        event::KeyCode::Down => {
                            let max = match app.selected_menu {
                                0 => 6, // Matematika
                                1 => 4, // Fisika
                                2 => 4, // Kimia
                                _ => 1,
                            };
                            app.selected_submenu = (app.selected_submenu + 1) % max;
                        }
                        event::KeyCode::Up => {
                            let max = match app.selected_menu {
                                0 => 6, 1 | 2 => 4, _ => 1
                            };
                            if app.selected_submenu == 0 {
                                app.selected_submenu = max - 1;
                            } else {
                                app.selected_submenu -= 1;
                            }
                        }
                        event::KeyCode::Enter => {
                            app.state = AppState::Fungsi;
                            app.selected_func = 0;
                        }
                        _ => {}
                    },
                    AppState::Fungsi => match key.code {
                        event::KeyCode::Esc => {
                            app.state = AppState::SubMenu;
                        }
                        event::KeyCode::Down => {
                            let max = match app.selected_menu {
                                0 => match app.selected_submenu {
                                    0 => 15, 1 => 14, 2 => 8, 3 => 5, 4 => 5, 5 => 2, _ => 1
                                },
                                1 => match app.selected_submenu { 0 => 2, 1 => 1, 2 => 2, 3 => 3, _ => 1 },
                                2 => match app.selected_submenu { 0 => 1, 1 => 2, 2 => 2, 3 => 1, _ => 1 },
                                _ => 1
                            };
                            app.selected_func = (app.selected_func + 1) % max;
                        }
                        event::KeyCode::Up => {
                            let max = match app.selected_menu {
                                0 => match app.selected_submenu {
                                    0 => 15, 1 => 14, 2 => 8, 3 => 5, 4 => 5, 5 => 2, _ => 1
                                },
                                1 => match app.selected_submenu { 0 => 2, 1 => 1, 2 => 2, 3 => 3, _ => 1 },
                                2 => match app.selected_submenu { 0 => 1, 1 => 2, 2 => 2, 3 => 1, _ => 1 },
                                _ => 1
                            };
                            if app.selected_func == 0 {
                                app.selected_func = max - 1;
                            } else {
                                app.selected_func -= 1;
                            }
                        }
                        event::KeyCode::Enter => {
                            app.state = AppState::Input;
                            app.input.clear();
                            app.output.clear();
                            // Set current_func sesuai menu/submenu/fungsi
                            match app.selected_menu {
                                0 => { // Matematika
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
                                1 => { // Fisika
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
                                2 => { // Kimia
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
                        }
                        _ => {}
                    },
                    AppState::Input => match key.code {
                        event::KeyCode::Esc => {
                            app.state = AppState::Fungsi;
                        }
                        event::KeyCode::Char(c) => {
                            if c.is_ascii_digit() || c == '.' || c == '-' || c == ',' || c == '[' || c == ']' {
                                app.input.push(c);
                            }
                        }
                        event::KeyCode::Backspace => {
                            app.input.pop();
                        }
                        event::KeyCode::Enter => {
                            // Proses input sesuai fungsi
                            if let Some(func) = app.current_func_mtk {
                                use matematika::*;
                                app.output = match func {
                                    MatematikaFunc::Pangkat => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(base), Ok(exp)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<u32>()) {
                                                let hasil = aritmetika::pangkat(base, exp);
                                                format!("{}^{} = {}", base, exp, hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: base,eksponen".to_string() }
                                    }
                                    MatematikaFunc::AkarKuadrat => {
                                        if let Ok(x) = app.input.trim().parse::<f64>() {
                                            let hasil = aritmetika::akar_kuadrat(x);
                                            format!("√{} = {}", x, hasil)
                                        } else { "Input tidak valid".to_string() }
                                    }
                                    MatematikaFunc::Faktorial => {
                                        if let Ok(n) = app.input.trim().parse::<u64>() {
                                            let hasil = kombinatorika::faktorial(n);
                                            format!("{}! = {}", n, hasil)
                                        } else { "Input tidak valid".to_string() }
                                    }
                                    MatematikaFunc::Kombinasi => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(n), Ok(k)) = (parts[0].trim().parse::<u64>(), parts[1].trim().parse::<u64>()) {
                                                let hasil = kombinatorika::kombinasi(n, k);
                                                format!("C({}, {}) = {}", n, k, hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: n,k".to_string() }
                                    }
                                    MatematikaFunc::Permutasi => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(n), Ok(r)) = (parts[0].trim().parse::<u64>(), parts[1].trim().parse::<u64>()) {
                                                let hasil = kombinatorika::permutasi(n, r);
                                                format!("P({}, {}) = {}", n, r, hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: n,r".to_string() }
                                    }
                                    MatematikaFunc::Mean => {
                                        let data: Vec<f64> = app.input.trim().trim_matches(['[',']'].as_ref()).split(',').filter_map(|x| x.trim().parse().ok()).collect();
                                        if !data.is_empty() {
                                            let hasil = statistika::Statistika::mean(&data);
                                            format!("Mean = {}", hasil)
                                        } else { "Input: [x1,x2,...]".to_string() }
                                    }
                                    MatematikaFunc::Median => {
                                        let mut data: Vec<f64> = app.input.trim().trim_matches(['[',']'].as_ref()).split(',').filter_map(|x| x.trim().parse().ok()).collect();
                                        if !data.is_empty() {
                                            let hasil = statistika::Statistika::median(&mut data);
                                            format!("Median = {}", hasil)
                                        } else { "Input: [x1,x2,...]".to_string() }
                                    }
                                    MatematikaFunc::Modus => {
                                        let data: Vec<i64> = app.input.trim().trim_matches(['[',']'].as_ref()).split(',').filter_map(|x| x.trim().parse().ok()).collect();
                                        if !data.is_empty() {
                                            let hasil = statistika::Statistika::modus(&data);
                                            format!("Modus = {:?}", hasil)
                                        } else { "Input: [x1,x2,...]".to_string() }
                                    }
                                    MatematikaFunc::Varian => {
                                        let data: Vec<f64> = app.input.trim().trim_matches(['[',']'].as_ref()).split(',').filter_map(|x| x.trim().parse().ok()).collect();
                                        if !data.is_empty() {
                                            let hasil = statistika::Statistika::varian(&data);
                                            format!("Varian = {}", hasil)
                                        } else { "Input: [x1,x2,...]".to_string() }
                                    }
                                    MatematikaFunc::StandarDeviasi => {
                                        let data: Vec<f64> = app.input.trim().trim_matches(['[',']'].as_ref()).split(',').filter_map(|x| x.trim().parse().ok()).collect();
                                        if !data.is_empty() {
                                            let hasil = statistika::Statistika::standar_deviasi(&data);
                                            format!("Standar Deviasi = {}", hasil)
                                        } else { "Input: [x1,x2,...]".to_string() }
                                    }
                                    // Tambahkan implementasi fungsi lain sesuai kebutuhan
                                    _ => format!("[Matematika] Fungsi: {:?}, Input: {}", func, app.input)
                                };
                            } else if let Some(func) = app.current_func_fisika {
                                use fisika::*;
                                app.output = match func {
                                    FisikaFunc::EnergiKinetik => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(m), Ok(v)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = energi::energi_kinetik(m, v);
                                                format!("Energi Kinetik = {} J", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: m,v".to_string() }
                                    }
                                    FisikaFunc::EnergiPotensial => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 3 {
                                            if let (Ok(m), Ok(g), Ok(h)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>()) {
                                                let hasil = energi::energi_potensial(m, g, h);
                                                format!("Energi Potensial = {} J", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: m,g,h".to_string() }
                                    }
                                    FisikaFunc::Gaya => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(m), Ok(a)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = gaya::gaya(m, a);
                                                format!("Gaya = {} N", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: m,a".to_string() }
                                    }
                                    FisikaFunc::GLBBPerpindahan => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 3 {
                                            if let (Ok(v0), Ok(t), Ok(a)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>()) {
                                                let hasil = gerak::glbb_perpindahan(v0, t, a);
                                                format!("Perpindahan = {} m", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: v0,t,a".to_string() }
                                    }
                                    FisikaFunc::GLBBKecepatanAkhir => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 3 {
                                            if let (Ok(v0), Ok(a), Ok(t)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>()) {
                                                let hasil = gerak::glbb_kecepatan_akhir(v0, a, t);
                                                format!("Kecepatan Akhir = {} m/s", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: v0,a,t".to_string() }
                                    }
                                    FisikaFunc::OhmTegangan => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(i), Ok(r)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = listrik::ohm_tegangannya(i, r);
                                                format!("Tegangan = {} V", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: i,r".to_string() }
                                    }
                                    FisikaFunc::OhmArus => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(v), Ok(r)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = listrik::ohm_arusnya(v, r);
                                                format!("Arus = {} A", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: v,r".to_string() }
                                    }
                                    FisikaFunc::OhmHambatan => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(v), Ok(i)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = listrik::ohm_hambatannya(v, i);
                                                format!("Hambatan = {} Ohm", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: v,i".to_string() }
                                    }
                                    _ => format!("[Fisika] Fungsi: {:?}, Input: {}", func, app.input)
                                };
                            } else if let Some(func) = app.current_func_kimia {
                                use kimia::*;
                                app.output = match func {
                                    KimiaFunc::TekananGasIdeal => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 4 {
                                            if let (Ok(n), Ok(r), Ok(t), Ok(v)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>(), parts[3].trim().parse::<f64>()) {
                                                let hasil = gas::tekanan_gas_ideal(n, r, t, v);
                                                format!("Tekanan Gas Ideal = {} atm", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: n,r,t,v".to_string() }
                                    }
                                    KimiaFunc::Molaritas => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(n), Ok(v)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = larutan::molaritas(n, v);
                                                format!("Molaritas = {} M", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: n,V".to_string() }
                                    }
                                    KimiaFunc::PHAsamKuat => {
                                        if let Ok(h) = app.input.trim().parse::<f64>() {
                                            let hasil = larutan::ph_asam_kuat(h);
                                            format!("pH = {}", hasil)
                                        } else { "Input tidak valid".to_string() }
                                    }
                                    KimiaFunc::MassaProduk => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(n), Ok(mr)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = reaksi::massa_produk(n, mr);
                                                format!("Massa Produk = {} gram", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: n,Mr".to_string() }
                                    }
                                    KimiaFunc::PersenHasil => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(aktual), Ok(teoritis)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = reaksi::persen_hasil(aktual, teoritis);
                                                format!("Persen Hasil = {}%", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: aktual,teoritis".to_string() }
                                    }
                                    KimiaFunc::JumlahMol => {
                                        let parts: Vec<&str> = app.input.split(',').collect();
                                        if parts.len() == 2 {
                                            if let (Ok(massa), Ok(mr)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                                                let hasil = stoikiometri::jumlah_mol(massa, mr);
                                                format!("Jumlah Mol = {} mol", hasil)
                                            } else { "Input tidak valid".to_string() }
                                        } else { "Format input: massa,Mr".to_string() }
                                    }
                                    _ => format!("[Kimia] Fungsi: {:?}, Input: {}", func, app.input)
                                };
                            } else {
                                app.output = "Fungsi belum dipilih".to_string();
                            }
                            app.state = AppState::Output;
                        }
                        _ => {}
                    },
                    AppState::Output => match key.code {
                        event::KeyCode::Esc | event::KeyCode::Enter => {
                            app.state = AppState::Fungsi;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
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
    let tabs = Tabs::new(menu_titles.iter().cloned().map(Spans::from).collect())
        .select(app.selected_menu)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, layout[1]);

    match app.state {
        AppState::Menu => {
            let help = Paragraph::new("Gunakan panah kiri/kanan untuk memilih menu utama, Enter untuk masuk submenu, q untuk keluar.")
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
            let items: Vec<ListItem> = submenu_titles.iter().enumerate().map(|(i, t)| {
                if i == app.selected_submenu {
                    ListItem::new(format!("> {}", t)).style(Style::default().fg(Color::Yellow))
                } else {
                    ListItem::new(format!("  {}", t))
                }
            }).collect();
            let submenu = List::new(items)
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
            let items: Vec<ListItem> = fungsi_titles.iter().enumerate().map(|(i, t)| {
                if i == app.selected_func {
                    ListItem::new(format!("> {}", t)).style(Style::default().fg(Color::Yellow))
                } else {
                    ListItem::new(format!("  {}", t))
                }
            }).collect();
            let fungsi = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Fungsi"));
            f.render_widget(fungsi, layout[2]);
        }
        AppState::Input => {
            let input = Paragraph::new(app.input.as_ref())
                .block(Block::default().borders(Borders::ALL).title("Input (tekan Enter untuk submit, ESC untuk kembali)"));
            f.render_widget(input, layout[2]);
        }
        AppState::Output => {
            let output = Paragraph::new(app.output.as_ref())
                .block(Block::default().borders(Borders::ALL).title("Hasil (Enter/ESC untuk kembali)"));
            f.render_widget(output, layout[2]);
        }
    }
}
