# 📦 Sistem Laporan Inventaris Toko Sederhana

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat&logo=rust)
![Status](https://img.shields.io/badge/Status-Completed-success)
![License](https://img.shields.io/badge/License-MIT-blue)

Program Rust sederhana untuk mencatat dan melaporkan kondisi inventaris barang toko. Project ini dibangun sebagai latihan fundamental Rust dengan fokus pada manajemen **ownership**, **type conversion**, dan struktur data dasar tanpa menggunakan kontrol alur kompleks.

---

## 📋 Tentang Project

Sebuah toko kecil membutuhkan sistem untuk mencetak laporan inventaris dari data yang sudah disiapkan. Program ini menyimpan data 3 jenis barang, menghitung total harga per barang, serta menghitung nilai total inventaris beserta pajaknya, lalu menampilkan laporan terformat ke terminal.

### 🎯 Tujuan Pembelajaran
Project ini dirancang untuk melatih pemahaman materi dasar Rust:
- ✅ **Ownership & Borrowing** pada `Array of String`
- ✅ **Type Conversion** (`as` casting) antar tipe numerik
- ✅ Penggunaan **Tuple** untuk pengelompokan data
- ✅ Manipulasi **Array** tanpa loop
- ✅ Format output dengan `println!` macro

---

## 🚀 Fitur

- **Laporan Terstruktur**: Menampilkan daftar barang dengan nama, stok, harga satuan, dan total harga.
- **Kalkulasi Otomatis**: 
  - Total harga per barang (`stok × harga`)
  - Total nilai inventaris seluruh barang
  - Perhitungan pajak inventaris
- **Tanpa Input User**: Data disiapkan secara *hardcoded* sesuai spesifikasi.
- **Output Bersih**: Laporan diformat rapi tanpa karakter debug.

---

## 🛠️ Tech Stack & Batasan

| Komponen | Keterangan |
|----------|------------|
| **Bahasa** | Rust |
| **Struktur Data** | `Array`, `Tuple`, `String`, `const` |
| **Tipe Data** | `String`, `i8`, `i32`, `f32` |
| **Batasan** | ❌ Tanpa `loop`, `for`, `while`<br>❌ Tanpa `if/else`<br>❌ Tanpa `Vec` atau struct |

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

[1] Nama Barang: indomie
    Stok: 30
    Harga satuan: 5000
    Total Harga: 150000

[2] Nama Barang: Beras
    Stok: 45
    Harga satuan: 43000
    Total Harga: 1935000

[3] Nama Barang: Gula
    Stok: 12
    Harga satuan: 12000
    Total Harga: 144000

Batas Maksimum Stock     : 50.0 pcs
Total Inventaris         : Rp 2229000
Pajak                    : 2 %
Total Inventaris + Pajak : Rp 2273580
==================================================
```

---

## 🧠 Poin Teknis Penting

### 1. Ownership pada Array of String
Menggunakan `&items[i]` untuk meminjam referensi string tanpa memindahkan ownership, sehingga array tetap bisa digunakan berulang kali:
```rust
let item1: &String = &items[0]; // Borrow, bukan move
```

### 2. Type Conversion
Konversi tipe data eksplisit untuk kalkulasi antar tipe berbeda:
```rust
let total_price1: i32 = stock1 as i32 * price1;
```

### 3. Struktur Tuple
Tuple digunakan untuk mengelompokkan ringkasan data setiap barang:
```rust
let summary: (
    (String, String, String, String),
    (String, String, String, String),
    (String, String, String, String),
) = ( ... );
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

## 📝 Catatan Pengembangan

Project ini sengaja dibuat tanpa `loop` dan `if/else` sebagai latihan fundamental. Untuk pengembangan lebih lanjut, dapat ditambahkan:
- [ ] Input dinamis dari user
- [ ] Penggunaan `Vec` untuk jumlah barang fleksibel
- [ ] Loop untuk iterasi yang lebih efisien
- [ ] Conditional untuk status stok (Aman/Kritis)
- [ ] Export laporan ke file

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
```

### 💡 Tips Tambahan untuk Repository Kamu:

1.  **Simpan README**: Buat file bernama `README.md` di folder utama repository dan paste konten di atas.
2.  **Tambahkan `.gitignore`**: Pastikan file `target/` tidak ter-commit dengan membuat file `.gitignore`:
    ```
    target/
    Cargo.lock
    ```
3.  **Cek Cargo.toml**: Pastikan `Cargo.toml` kamu memiliki deskripsi yang sesuai:
    ```toml
    [package]
    name = "sistem_laporan_inventaris_toko_sederhana"
    version = "0.1.0"
    edition = "2021"
    description = "Sistem laporan inventaris toko sederhana sebagai latihan fundamental Rust"
