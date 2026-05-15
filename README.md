# Sistem Monitoring dan Kontrol Suhu Kolam Ikan Lele Berbasis Rust

Project ini dibuat untuk ETS mata kuliah Algoritma dan Pemrograman Teknik Instrumentasi. Aplikasi berbasis terminal ini mensimulasikan sistem instrumentasi sederhana untuk memonitor dan mengontrol suhu air pada industri budidaya ikan lele modern.

## Latar Belakang

Suhu air merupakan parameter penting dalam budidaya ikan lele karena memengaruhi pertumbuhan, metabolisme, konsumsi pakan, stres, daya tetas telur, kelangsungan hidup, kualitas air, dan produktivitas budidaya. Berdasarkan studi yang digunakan dalam project, suhu optimal budidaya lele berada pada kisaran 26-30 C. Sistem ini dibuat untuk menjaga suhu tetap berada pada kondisi aman melalui monitoring suhu, klasifikasi kondisi, kontrol heater, kontrol aerator/cooling system, alarm, serta moving average untuk menstabilkan pembacaan sensor.

## Fitur Program

- Input suhu air kolam sebagai simulasi sensor virtual.
- Validasi input agar tidak menerima data kosong, huruf, atau suhu tidak realistis.
- Klasifikasi kondisi suhu: Dingin, Normal, Panas, Bahaya.
- Heater otomatis ON saat suhu di bawah 24 C.
- Aerator/cooling system otomatis ON saat suhu di atas 30 C.
- Alarm aktif saat suhu di atas 35 C.
- Monitoring realtime sederhana menggunakan loop.
- Histori suhu selama program berjalan.
- Moving average 5 data terakhir untuk mengurangi noise pembacaan sensor.
- Implementasi OOP Rust menggunakan `struct`, `impl`, dan `method`.

## Batas Keputusan Sistem

| Rentang Suhu | Kondisi | Aksi Sistem |
|---|---|---|
| < 24 C | Dingin | Heater ON |
| 24-30 C | Normal | Semua aktuator normal/OFF |
| > 30 C | Panas | Aerator/Cooling ON |
| > 35 C | Bahaya | Aerator/Cooling ON dan Alarm AKTIF |

## Struktur Project

```text
monitoring_suhu_kolam_lele/
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
├── docs/
│   ├── flowchart.dot
│   ├── flowchart.md
│   └── laporan_ets_lele.tex
├── assets/
│   ├── flowchart.png
│   └── screenshot_output.png
└── tests_manual/
    └── sample_input_output.txt
```

## Cara Menjalankan Program

Pastikan Rust sudah terpasang. Jalankan perintah berikut:

```bash
cargo run
```

Masukkan suhu air kolam dalam satuan Celsius. Ketik `q` untuk keluar.

## Contoh Output Program

![Screenshot Output](assets/screenshot_output.png)

## Flowchart Sistem

![Flowchart Sistem](assets/flowchart.png)

## Konsep OOP Rust

Program menggunakan tiga object utama:

1. `Sensor`: menyimpan nama sensor dan nilai suhu.
2. `Controller`: menangani logika heater, cooling system, dan alarm.
3. `MonitoringSystem`: menjalankan proses input, histori, moving average, klasifikasi, dan output monitoring.

## Komputasi Numerik

Metode numerik yang digunakan adalah moving average dengan window 5 data terakhir. Rumus:

```text
MA = (x1 + x2 + ... + xn) / n
```

Moving average digunakan agar data suhu lebih stabil dan tidak terlalu sensitif terhadap noise pembacaan sensor.

## Dokumen Pendukung

- Source code Rust: `src/main.rs`
- Flowchart: `docs/flowchart.md`, `assets/flowchart.png`
- Laporan LaTeX: `docs/laporan_ets_lele.tex`
- PDF laporan: `laporan_ets_lele.pdf`
- Contoh pengujian: `tests_manual/sample_input_output.txt`

## Catatan Repository GitHub

Repository harus dibuat public dan minimal memuat source code Rust, README.md, flowchart, dokumentasi, screenshot output, file laporan LaTeX, dan PDF laporan. Commit harus dilakukan bertahap agar aktivitas pengembangan terlihat.
