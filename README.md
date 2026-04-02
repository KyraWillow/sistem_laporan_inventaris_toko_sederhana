# 📦 Sistem Laporan Inventaris Toko Sederhana

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat&logo=rust)
![Status](https://img.shields.io/badge/Status-Completed-success)
![License](https://img.shields.io/badge/License-MIT-blue)

Program Rust sederhana untuk mencatat dan melaporkan kondisi inventaris barang toko. Project ini dibangun sebagai latihan fundamental Rust dengan fokus pada manajemen **ownership & borrowing**, **control flow**, **looping**, dan struktur data dasar.

---

## 📋 Tentang Project

Sebuah toko kecil membutuhkan sistem untuk mencetak laporan inventaris dari data yang sudah disiapkan. Program ini menyimpan data 3 jenis barang, menghitung total harga per barang, menentukan status stok secara otomatis, serta menghitung nilai total inventaris beserta pajaknya — lalu menampilkan laporan terformat ke terminal.

### 🎯 Tujuan Pembelajaran
Project ini dirancang untuk melatih pemahaman materi dasar Rust:
- ✅ **Ownership & Borrowing** pada `Array of String`
- ✅ **Type Conversion** (`as` casting) antar tipe numerik
- ✅ **For Loop** untuk iterasi array dengan index
- ✅ **If Expression** untuk keputusan status stok
- ✅ **Augmented Assignment** untuk akumulasi nilai di dalam loop
- ✅ Penggunaan `const` sebagai batas referensi
- ✅ Format output dengan `println!` macro

---

## 🚀 Fitur

- **Laporan Terstruktur**: Menampilkan daftar barang dengan nama, stok, harga satuan, total harga, dan status stok.
- **Kalkulasi Otomatis**:
  - Total harga per barang (`stok × harga`)
  - Total nilai inventaris seluruh barang (akumulasi dalam loop)
  - Perhitungan pajak inventaris
- **Status Stok Dinamis**: Setiap barang otomatis mendapat status berdasarkan perbandingan stok aktual terhadap batas maksimum.
- **Tanpa Input User**: Data disiapkan secara *hardcoded* sesuai spesifikasi.
- **Output Bersih**: Laporan diformat rapi dan terstruktur.

---

## 🛠️ Tech Stack & Batasan

| Komponen | Keterangan |
|----------|------------|
| **Bahasa** | Rust |
| **Struktur Data** | `Array`, `String`, `const` |
| **Tipe Data** | `String`, `u32`, `i8` |
| **Control Flow** | `for` loop, `if` expression |
| **Batasan** | ❌ Tanpa `Vec` atau `struct` |

---

## 📥 Instalasi & Penggunaan

### Prasyarat
Pastikan Rust sudah terinstal di sistem Anda:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone Repository
```bash
git clone https://github.com/KyraWillow/sistem_laporan_inventaris_toko_sederhana.git
cd sistem_laporan_inventaris_toko_sederhana
```

### Jalankan Program
```bash
cargo run
```

### Build Release
```bash
cargo build --release
```

---

## 📊 Contoh Output

```text
===== LAPORAN INVENTARIS TOKO =====

[1]  Nama Barang                : indomie
      Stock                     : 30
      Harga satuan              : 5000
      Total harga               : 150000
      Status Stok               : Aman

[2]  Nama Barang                : Beras
      Stock                     : 45
      Harga satuan              : 43000
      Total harga               : 1935000
      Status Stok               : Aman

[3]  Nama Barang                : Gula
      Stock                     : 12
      Harga satuan              : 12000
      Total harga               : 144000
      Status Stok               : Segera Restock

===== RINGKASAN =====
      Batas Maksimum Stock      : 50 pcs
      Total Inventaris          : Rp 2229000
      Pajak                     : 2%
      Total Inventaris + Pajak  : Rp 2273580
====================================
```

---

## 🧠 Poin Teknis Penting

### 1. Ownership pada Array of String
Menggunakan `&items[i]` untuk meminjam referensi string tanpa memindahkan ownership, sehingga array tetap valid sepanjang program berjalan:
```rust
// Borrow via referensi, bukan move — owner tetap `items`
println!("{}", &items[i]);
```

### 2. Iterasi Array dengan Index
Menggunakan `for i in 0..3` untuk mengakses ketiga array sekaligus dalam satu iterasi — menghindari repetisi kode manual:
```rust
for i in 0..3 {
    println!("{}", &items[i]);
    println!("{}", &stocks[i]);
    println!("{}", &prices[i]);
}
```

### 3. Akumulasi dalam Loop
Variable `total_inventaris` dideklarasikan `mut` di luar loop lalu diakumulasikan di dalam — pola umum untuk menghitung total dari koleksi data:
```rust
let mut total_inventaris: u32 = 0;
for i in 0..3 {
    total_inventaris += &prices[i] * stocks[i];
}
```

### 4. If Expression untuk Status Stok
`if` digunakan sebagai *expression* yang menghasilkan value `&str`, lalu disimpan ke variable sebelum dicetak — memisahkan logika dari presentasi:
```rust
let status: &str = if stocks[i] >= MAKSIMUM_STOCK / 2 {
    "Aman"
} else if stocks[i] >= MAKSIMUM_STOCK / 5 {
    "Segera Restock"
} else {
    "Kritis"
};
println!("Status Stok: {status}");
```

### 5. Const sebagai Referensi Batas
`MAKSIMUM_STOCK` tidak dipakai sebagai stok aktual, melainkan sebagai pembanding untuk menentukan status — sesuai peruntukannya sebagai batas maksimum:
```rust
const MAKSIMUM_STOCK: u32 = 50;
const PAJAK: i8 = 2;
```

---

## 📁 Struktur Project

```
sistem_laporan_inventaris_toko_sederhana/
├── src/
│   └── main.rs          # Source code utama
├── Cargo.toml           # Manifest file
└── README.md            # Dokumentasi project
```

---

## 🗺️ Riwayat Pengembangan

| Versi | Deskripsi |
|-------|-----------|
| **v1** | Akses array manual per index, kalkulasi tanpa loop, tanpa status stok |
| **v2** | Refactor menggunakan `for` loop, tambah `if` expression untuk status stok, akumulasi total dalam loop |

---

## 📝 Catatan Pengembangan

Untuk pengembangan lebih lanjut, dapat ditambahkan:
- [ ] Input dinamis dari user (`stdin`)
- [ ] Penggunaan `Vec` untuk jumlah barang yang fleksibel
- [ ] `struct` untuk merepresentasikan satu barang secara utuh
- [ ] Export laporan ke file teks

---

## 👤 Penulis

**KyraWillow**
- GitHub: [@KyraWillow](https://github.com/KyraWillow)
- Repository: [sistem_laporan_inventaris_toko_sederhana](https://github.com/KyraWillow/sistem_laporan_inventaris_toko_sederhana)

---

## 📄 Lisensi

Project ini dilisensikan di bawah [MIT License](LICENSE).

---

⭐ *Jika project ini membantu pembelajaran Anda, jangan lupa berikan star!*