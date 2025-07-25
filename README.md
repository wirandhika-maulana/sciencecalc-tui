# sciencecalc-tui

Terminal User Interface (TUI) untuk library [sciencecalc-rs](https://github.com/wirandhika-maulana/sciencecalc-rs).

## Fitur
- Menu utama: Matematika, Fisika, Kimia
- Submenu sesuai kategori (misal: Aljabar, Basis, Geometri, dll untuk Matematika)
- Menampilkan semua fungsi/perhitungan dari library `sciencecalc-rs` sesuai submenu
- Navigasi keyboard (panah, enter, esc, q untuk keluar)


## Cara Menjalankan

### Jalankan Langsung (Mode Development)
```bash
cd sciencecalc-tui
cargo run
```

### Build File .exe (Windows)
1. Pastikan sudah terinstall Rust toolchain (https://www.rust-lang.org/tools/install)
2. Buka terminal di folder `sciencecalc-tui`
3. Jalankan perintah berikut:
```bash
cargo build --release
```
4. File executable `.exe` akan muncul di folder `target/release/` dengan nama `sciencecalc-tui.exe`
5. Jalankan dengan:
```bash
target\release\sciencecalc-tui.exe
```

### Catatan
- Untuk pengalaman terbaik, gunakan terminal yang mendukung warna (Windows Terminal, PowerShell, cmd baru, dsb).
- Jika ingin logging ke file, atur environment variable `RUST_LOG` dan gunakan crate log/env_logger.

## Gambaran TUI
```
┌─────────────────────────────── SCIENCECALC ────────────────────────────────┐
│░██████╗░█████╗░██╗███████╗███╗░░██╗░█████╗░███████╗░█████╗░█████╗░██╗░░░░│
│██╔════╝██╔══██╗██║██╔════╝████╗░██║██╔══██╗██╔════╝██╔══██╗██╔══██╗██║░░░░│
│╚█████╗░██║░░╚═╝██║█████╗░░██╔██╗██║██║░░╚═╝█████╗░░██║░░╚═╝███████║██║░░░░│
│░╚═══██╗██║░░██╗██║██╔══╝░░██║╚████║██║░░██╗██╔══╝░░██║░░██╗██╔══██║██║░░░░│
│██████╔╝╚█████╔╝██║███████╗██║░╚███║╚█████╔╝███████╗╚█████╔╝██║░░██║███████│
│╚═════╝░░╚════╝░╚═╝╚══════╝╚═╝░░╚══╝░╚════╝░╚══════╝░╚════╝░╚═╝░░╚═╝╚═════╝│
│                    S C I E N C E   C A L C U L A T O R                    │
│----------------------------------------------------------------------------│
│ Wirandhika Maulana (2025) sciencecalc-tui v0.1.0                           │
└─────────────────────────────────────────────────────────────────────────────┘
┌─────────────┐
│   Menu      │
├─────────────┤
│ Matematika  │
│ Fisika      │
│ Kimia       │
└─────────────┘

Jika memilih Matematika:
┌─────────────┐
│ Matematika  │
├─────────────┤
│ Aljabar     │
│ Basis       │
│ Geometri    │
│ Kombinatorika│
│ Statistika  │
│ Aritmetika  │
└─────────────┘

Jika memilih Fisika:
┌─────────────┐
│   Fisika    │
├─────────────┤
│ Energi      │
│ Gaya        │
│ Gerak       │
│ Listrik     │
└─────────────┘

Jika memilih Kimia:
┌─────────────┐
│   Kimia     │
├─────────────┤
│ Gas         │
│ Larutan     │
│ Reaksi      │
│ Stoikiometri│
└─────────────┘

Jika memilih salah satu submenu, akan muncul daftar fungsi/perhitungan dari library sesuai kategori.
```

## Referensi
- [sciencecalc-rs](https://github.com/wirandhika-maulana/sciencecalc-rs)
- [ratatui](https://github.com/ratatui-org/ratatui)
