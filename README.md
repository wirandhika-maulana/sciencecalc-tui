[![MIT License](https://img.shields.io/github/license/wirandhika-maulana/sciencecalc-tui)](https://github.com/wirandhika-maulana/sciencecalc-tui/blob/master/LICENSE)


<div align="center">
  <h1>📐 SCIENCE CALC TUI</h1>
 
  
  <p>
    <strong>A CALCULATOR TOOLKIT FOR MATEMATIKA, FISIKA, KIMIA</strong><br>
    Built with Rust, Ratatui, and Crossterm
  </p>
  
  <p>
    <a href="https://crates.io/crates/sciencecalc-tui" target="_blank">
      <img src="https://img.shields.io/badge/📦_Crate-spltui-orange?style=for-the-badge&logo=rust" alt="Crate">
    </a>
    <a href="#-getting-started">
      <img src="https://img.shields.io/badge/📖_Get_Started-Documentation-green?style=for-the-badge&logo=gitbook" alt="Documentation">
    </a>
    <a href="#-tech-stack">
      <img src="https://img.shields.io/badge/⚙️_Tech_Stack-Rust-orange?style=for-the-badge&logo=rust" alt="Tech Stack">
    </a>
</div>

---

<img src="https://github.com/wirandhika-maulana/sciencecalc-tui/blob/master/ssciencecalc-tui.png" alt="SCIENCECAL TUI Screenshot" style="max-width: 60%; border-radius: 12px; margin-top: 16px;" />

## 👋 About

**SPLTUI** (Sistem Persamaan Linear Terminal User Interface, atau SPLTUI, merupakan sebuah aplikasi kalkulator untuk menghitung sistem persamaan linear dengan berbagai variabel berbasis **terminal user interface** atau TUI.  
Dibangun oleh **Wirandhika Maulana Akbar** dengan Rust dan framework TUI modern.

## 🚀 Live Demo

📹 **Video Demonstrasi:** [Dokumentasi - SCIENCECALC-TUI Demo]([https://github.com/wirandhika-maulana/sciencecalc-tui])

## 🛠️ Tech Stack

<div align="center">

| Language | TUI | Terminal | Math Engine | Logging |
|----------|-----|----------|-------------|---------|
| ![Rust](https://img.shields.io/badge/Rust-orange?style=for-the-badge&logo=rust) | ![Ratatui](https://img.shields.io/badge/Ratatui-22C55E?style=for-the-badge&logo=terminal) | ![Crossterm](https://img.shields.io/badge/Crossterm-4B5563?style=for-the-badge&logo=terminal) | ![lib.matematika-rs](https://img.shields.io/badge/sciencecalc.rs-blueviolet?style=for-the-badge) | ![Log](https://img.shields.io/badge/Logging-env_logger-red?style=for-the-badge) |

</div>

### 🔧 Key Technologies

- **Rust** + Cargo
- **Ratatui** (TUI Framework)
- **Crossterm** (Terminal Handling)
- **matematika.rs** (Linear System Solver)
- **Log & env_logger**
- **Chrono** (Timestamp Logging)

## ✨ Features

- 📐 **SPLDV & SPLSV Support** (2 atau n variabel)
- 🌗 **Dark/Light Theme**
- 📂 **Verbose Mode & Logging**
- 🗂️ **State Navigation** (Input, Result, etc)
- ⚡ **Fast CLI Performance**
- 💻 **Cross Platform (Linux, Windows, Termux)**

## 📋 Prerequisites

<div align="center">

| Requirement | Version | Download |
|-------------|---------|----------|
| ![Rust](https://img.shields.io/badge/Rust-orange?style=for-the-badge&logo=rust) | Latest | [rustup.rs](https://rustup.rs/) |

</div>

## 🏃‍♂️ Getting Started

<details>
<summary><strong>📥 Step 1: Install via Cargo (Recommended)</strong></summary>

```bash
cargo install sciencecalc-tui
```

</details>

<details>
<summary><strong>⚙️ Step 2: Run SPLTUI</strong></summary>

```bash
sciencecalc-tui
```

</details>

## 🏗️ Building from Source

<details>
<summary><strong>🔨 Build Guide</strong></summary>

```bash
git clone https://github.com/wirandhika-maulana/sciencecalc-tui.git
cd sciencecalc-tui
cargo build --release
./target/release/sciencecalc-tui
```

</details>

## 🚨 Troubleshooting

<details>
<summary><strong>❌ Common Issues & Solutions</strong></summary>

| Problem | Solution |
|---------|----------|
| Rust not installed | Install via [rustup.rs](https://rustup.rs/) |
| `cargo install` error | Run `rustup update` |
| Terminal rendering issue | Use compatible terminal (e.g. Alacritty, Windows Terminal) |

</details>

## 🤝 Contributing

Contributions are welcome!

1. 🍴 Fork the repository
2. 🌟 Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. 💾 Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. 📤 Push to the branch (`git push origin feature/AmazingFeature`)
5. 🔄 Open a Pull Request


## Kompatibilitas

| Sistem Operasi | Kestabilan |
| :------------- | :--------: |
| Android (Termux) |       ✅       |
|     IOS (Ish)    |       🛠️       |
|       Linux      |       ✅       |
|       Windows    |       ✅       |
|       MacOS      |       🛠️       |


## 📞 Contact & Support

<div align="center">

### 👨‍💻 Wirandhika Maulana Akbar

<p>
  <a href="https://wirandhika.my.id/" target="_blank">
    <img src="https://img.shields.io/badge/🌐_Portfolio-wirandhika.my.id-blue?style=for-the-badge" alt="Portfolio">
  </a>
  <a href="https://github.com/wirandhika-maulana" target="_blank">
    <img src="https://img.shields.io/badge/GitHub-wirandhika-181717?style=for-the-badge&logo=github" alt="GitHub">
  </a>
  <a href="mailto:randikacreator22@gmail.com">
    <img src="https://img.shields.io/badge/Email-Contact-red?style=for-the-badge&logo=gmail&logoColor=white" alt="Email">
  </a>
  <a href="https://linkedin.com/in/wirandhika-maulana-akbar" target="_blank">
    <img src="https://img.shields.io/badge/LinkedIn-Connect-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn">
  </a>
</p>

</div>

---

**Made with ❤️ by [Wirandhika Maulana Akbar](https://wirandhika.my.id/)**  
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file.

> [!WARNING]
>
> REPOSITORI INI MASIH DALAM TAHAP PENGEMBANGAN.
>
> KESTABILAN DI DALAM *BRANCH* [`master`](https://github.com/wirandhika-maulana/sciencecalc-tui/tree/master) TIDAK DAPAT DIPASTIKAN!

---
