# 📦 crates.io — Repository Resmi Package Manager Rust

## 🔍 Apa itu crates.io?

[crates.io](https://crates.io) adalah **repository resmi untuk semua library (crate) yang digunakan dalam bahasa pemrograman Rust**.

Crate adalah unit terkecil dalam distribusi kode Rust — bisa berupa library (pustaka) atau binary (program). Dengan `crates.io`, pengembang dapat:
- Menemukan crate buatan orang lain
- Menggunakan crate tersebut di dalam proyek
- Mengunggah crate buatan sendiri agar bisa digunakan komunitas

---

## 🎯 Kegunaan crates.io

| Fitur                      | Penjelasan                                                                 |
|---------------------------|----------------------------------------------------------------------------|
| 🔍 Mencari crate           | Kamu bisa mencari pustaka Rust untuk berbagai kebutuhan seperti parsing, HTTP, math, dll. |
| 📦 Menggunakan crate       | Tambahkan crate ke `Cargo.toml`, dan langsung pakai dalam kode.            |
| 🔄 Mengatur versi crate    | Tersedia berbagai versi yang memudahkan manajemen dependency.              |
| 🚀 Upload crate sendiri    | Kamu bisa berbagi crate ke komunitas via `cargo publish`.                  |
| 📖 Dokumentasi otomatis    | crates.io terintegrasi dengan [docs.rs](https://docs.rs) untuk dokumentasi online. |
| 📊 Statistik penggunaan    | Lihat jumlah unduhan dan popularitas crate tertentu.                       |

---

## 🛠️ Contoh Penggunaan crates.io

### 1. Cari Crate

Kunjungi [https://crates.io](https://crates.io) dan cari, misalnya: `rand`

### 2. Tambahkan ke `Cargo.toml`

```toml
[dependencies]
rand = "0.8"