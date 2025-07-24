use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, FocusState, InputState};
use crate::menu::*;

/// Gambar header ASCII-art dan judul
pub fn draw_header<B: Backend>(f: &mut Frame<B>) {
    let area = f.size();
    let header = vec![
        Spans::from(" ____  _                 _         _    "),
        Spans::from("/ ___|| |__   ___   ___ | | ____ _| |_ "),
        Spans::from("\\___ \\| '_ \\ / _ \\ / _ \\| |/ / _` | __|"),
        Spans::from(" ___) | | | | (_) | (_) |   < (_| | |_ "),
        Spans::from("|____/|_| |_|\\___/ \\___/|_|\\_\\__,_|\\__|"),
        Spans::from("      SCIENCECALC-TUI                   "),
    ];
    let paragraph = Paragraph::new(header)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(paragraph, Rect::new(area.x, area.y, area.width, 7));
}

/// Gambar seluruh main UI sesuai state aplikasi
pub fn draw_main_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = f.size();

    // Layout utama: header, konten, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // header
            Constraint::Min(12),   // isi utama
            Constraint::Length(2), // footer
        ])
        .split(area);

    // Gambar menu berdasarkan focus
    match app.focus {
        FocusState::MenuUtama => draw_menu_utama(f, chunks[1], app),
        FocusState::SubMenu => draw_sub_menu(f, chunks[1], app),
        FocusState::SubSubMenu => draw_sub_sub_menu(f, chunks[1], app),
        FocusState::Input => draw_input_form(f, chunks[1], app),
        FocusState::Hasil => draw_hasil(f, chunks[1], app),
    }

    // Footer petunjuk navigasi
    let footer = Paragraph::new("Navigasi: ↑↓ Enter pilih, Esc kembali, q keluar")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}

/// Menu utama: Matematika, Fisika, Kimia
fn draw_menu_utama<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let items = ["Matematika", "Fisika", "Kimia"];
    let items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, &label)| {
            let style = if i == app.menu_utama.selected() {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(label).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Menu Utama"))
        .highlight_symbol("> ");
    f.render_widget(menu, area);
}

/// Sub-menu sesuai dengan MenuUtama yang dipilih
fn draw_sub_menu<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    if let Some(submenu) = &app.sub_menu {
        let (title, items, selected) = submenu.render_info();
        let items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(*label).style(style)
            })
            .collect();
        let menu = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_symbol("> ");
        f.render_widget(menu, area);
    }
}

/// Sub-sub-menu sesuai dengan SubMenu yang dipilih (jika ada)
fn draw_sub_sub_menu<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    if let Some(subsubmenu) = &app.sub_sub_menu {
        let (title, items, selected) = subsubmenu.render_info();
        let items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(*label).style(style)
            })
            .collect();
        let menu = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_symbol("> ");
        f.render_widget(menu, area);
    }
}

/// Form input interaktif (nanti bisa diperluas sesuai kebutuhan)
fn draw_input_form<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let mut lines = vec![];
    lines.push(Spans::from("Masukkan data perhitungan:"));
    if let Some(input) = &app.input_state {
        for (i, val) in input.fields.iter().enumerate() {
            let label = format!("Input {}:", i + 1);
            let style = if i == input.current_field {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            lines.push(Spans::from(vec![
                Span::styled(label, style),
                Span::raw(format!(" [{}]", val)),
            ]));
        }
    }
    lines.push(Spans::from(""));
    lines.push(Spans::from("Tekan Enter untuk submit, Esc untuk kembali."));
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .alignment(Alignment::Left);
    f.render_widget(para, area);
}

/// Menampilkan hasil perhitungan
fn draw_hasil<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let hasil = if let Some(res) = &app.result {
        res.clone()
    } else {
        "Tidak ada hasil.".to_string()
    };
    let para = Paragraph::new(hasil)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hasil Perhitungan")
                .style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(para, area);
}
