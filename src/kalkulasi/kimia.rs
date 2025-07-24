// src/kalkulasi/kimia.rs

// --- Re-export seluruh isi modul kimia dari sciencecalc-rs ---
pub use sciencecalc_rs::kimia::*;

// --- (Opsional) Wrapper fungsi/alias untuk nama ramah di TUI, atau langsung pakai fungsi aslinya ---

// Gas
pub fn tekanan_gas_ideal_p(n: f64, r: f64, t: f64, v: f64) -> f64 {
    tekanan_gas_ideal(n, r, t, v)
}

// Larutan
pub fn molaritas_m(n: f64, v: f64) -> f64 {
    molaritas(n, v)
}
pub fn ph_asam_kuat_val(h_concentration: f64) -> f64 {
    ph_asam_kuat(h_concentration)
}

// Reaksi
pub fn massa_produk_gr(n: f64, mr: f64) -> f64 {
    massa_produk(n, mr)
}
pub fn persen_hasil_yield(massa_aktual: f64, massa_teoritis: f64) -> f64 {
    persen_hasil(massa_aktual, massa_teoritis)
}

// Stoikiometri
pub fn jumlah_mol_n(massa: f64, massa_mr: f64) -> f64 {
    jumlah_mol(massa, massa_mr)
}

// ----------------------------------------------------------
// Semua fitur submodul berikut otomatis available:
// - Gas (tekanan_gas_ideal)
// - Larutan (molaritas, ph_asam_kuat)
// - Reaksi (massa_produk, persen_hasil)
// - Stoikiometri (jumlah_mol)
// ----------------------------------------------------------

// NB: Jika ingin menambah alias lain, tambahkan di sini sesuai kebutuhan TUI-mu.
// Seluruh fungsi aslinya tetap bisa dipanggil lewat pub use di atas.
