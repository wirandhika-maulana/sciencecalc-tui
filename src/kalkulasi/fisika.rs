// src/kalkulasi/fisika.rs

// Re-export seluruh isi modul fisika dari sciencecalc-rs
pub use sciencecalc_rs::fisika::*;

// ------- (Opsional) Wrapper fungsi/alias untuk TUI --------
// Kamu bisa menambah alias/namanya jika ingin nama user-friendly di interface, atau langsung pakai fungsi aslinya.

// Energi
pub fn energi_kinetik_kgms(m: f64, v: f64) -> f64 { energi_kinetik(m, v) }
pub fn energi_potensial_kgms(m: f64, g: f64, h: f64) -> f64 { energi_potensial(m, g, h) }

// Gaya
pub fn gaya_newton(m: f64, a: f64) -> f64 { gaya(m, a) }

// Gerak
pub fn glbb_perpindahan_meter(v0: f64, t: f64, a: f64) -> f64 { glbb_perpindcep_akhir_ms(v0: f64, a: f64, t: f64) -> f64 { glbb_kecepatan_akhir(v0, a, t) }

// Listrik (Hukum Ohm)
pub fn ohm_tegangannya_v(i: f64, r: f64) -> f64 { ohm_tegangannya(i, r) }
pub fn ohm_arusnya_a(v: f64, r: f64) -> f64 { ohm_arusnya(v, r) }
pub fn ohm_hambatannya_ohm(v: f64, i: f64) -> f64 { ohm_hambatannya(v, i) }

// ----------------------------------------------------------
// Semua fitur submodul berikut otomatis available:
// - Energi (energi_kinetik, energi_potensial)
// - Gaya (gaya)
// - Gerak (glbb_perpindahan, glbb_kecepatan_akhir)
// - Listrik (ohm_tegangannya, ohm_arusnya, ohm_hambatannya)
// ----------------------------------------------------------

// NB: Jika ingin menambah alias lain, tambahkan di sini sesuai kebutuhan TUI-mu.
// Seluruh fungsi aslinya tetap bisa dipanggil lewat pub use di atas.
