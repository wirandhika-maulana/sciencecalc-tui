/// State input untuk form TUI interaktif
#[derive(Debug, Clone)]
pub struct InputState {
    /// Field input (misal: ["", "", ...] sesuai kebutuhan form)
    pub fields: Vec<String>,
    /// Index field yang sedang diedit/aktif
    pub current_field: usize,
}

impl InputState {
    /// Buat input state baru dengan jumlah field tertentu
    pub fn with_fields(n: usize) -> Self {
        InputState {
            fields: vec![String::new(); n],
            current_field: 0,
        }
    }

    /// Pindah ke field berikutnya (jika belum di paling akhir)
    pub fn next_field(&mut self) {
        if self.current_field + 1 < self.fields.len() {
            self.current_field += 1;
        }
    }

    /// Pindah ke field sebelumnya (jika belum di paling awal)
    pub fn prev_field(&mut self) {
        if self.current_field > 0 {
            self.current_field -= 1;
        }
    }

    /// Tambah karakter ke field aktif (misal saat user mengetik)
    pub fn push_char(&mut self, c: char) {
        self.fields[self.current_field].push(c);
    }

    /// Hapus satu karakter (backspace) di field aktif
    pub fn pop_char(&mut self) {
        self.fields[self.current_field].pop();
    }

    /// Reset semua field ke kosong, kursor ke awal
    pub fn clear(&mut self) {
        for f in &mut self.fields {
            f.clear();
        }
        self.current_field = 0;
    }

    /// Apakah semua field sudah terisi (tidak kosong)?
    pub fn is_filled(&self) -> bool {
        self.fields.iter().all(|f| !f.trim().is_empty())
    }

    /// Ambil semua nilai field sebagai vektor f64 (jika valid)
    pub fn parse_f64(&self) -> Option<Vec<f64>> {
        let mut res = Vec::new();
        for s in &self.fields {
            match s.trim().replace(',', ".").parse::<f64>() {
                Ok(v) => res.push(v),
                Err(_) => return None,
            }
        }
        Some(res)
    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState::with_fields(4)
    }
}
