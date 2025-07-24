// src/kalkulasi/matematika.rs

// --- Re-export seluruh isi modul matematika dari sciencecalc-rs ---
pub use sciencecalc_rs::matematika::*;

// ---- (Opsional) Wrapper fungsi/alias jika ingin nama lebih ramah di TUI ----
// Atau langsung pakai dari modul aslinya, misal: Aljabar::spldv(...), dst.

// =====================
// Contoh fungsi helper alias (boleh dihapus jika tidak perlu)
// =====================
pub fn penjumlahan(a: f64, b: f64) -> f64 { tambah(a, b) }
pub fn pengurangan(a: f64, b: f64) -> f64 { kurang(a, b) }
pub fn perkalian(a: f64, b: f64) -> f64 { kali(a, b) }
pub fn pembagian(a: f64, b: f64) -> f64 { bagi(a, b) }
pub fn pangkat_f64(a: f64, b: u32) -> f64 { pangkat(a, b) }
pub fn akar(a: f64) -> f64 { akar_kuadrat(a) }
pub fn modulus(a: f64, b: f64) -> f64 { modulo(a, b) }
pub fn pembulatan(a: f64) -> f64 { bulat(a) }
pub fn nilai_absolut(a: f64) -> f64 { absolut(a) }
pub fn faktorial_u64(n: u64) -> u64 { faktorial(n) }
pub fn kombinasi_u64(n: u64, r: u64) -> u64 { kombinasi(n, r) }
pub fn permutasi_u64(n: u64, r: u64) -> u64 { permutasi(n, r) }

// =====================
// Contoh pemanggilan dari TUI:
// =====================
// Untuk struct ber-metode (OOP-style), langsung gunakan, misal:
// let spl = Aljabar::spldv(...);
// let persegi = bangun_datar::Persegi::new(5.0); persegi.luas();
// let hasil = Statistika::mean(&data);
// let hasil = basis::konversi_basis(15, 2);
// dst...

// Untuk trigonometri: Trigonometri::sin_deg(30.0);
// Untuk peluang: Peluang::Dadu, dst.

// =====================
// Semua fitur submodul berikut otomatis available:
// - Aljabar (SPL, Kuadrat, Matriks, Determinan, Invers, dst)
// - Aritmetika (operasi dasar, FPB, KPK, super array, Fibonacci, dst)
// - Basis (biner, oktal, desimal, heksa, konversi, dst)
// - Geometri (bidang/ruang, luas/keliling/volume, trigonometri, dst)
// - Kombinatorika (faktorial, permutasi, kombinasi, peluang, dst)
// - Statistika (mean, median, modus, varian, standar deviasi, dst)
// =====================

// NB: Jika mau override/tambah alias, tambahkan di sini sesuai kebutuhan TUI-mu.
