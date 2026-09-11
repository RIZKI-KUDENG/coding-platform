# Roadmap & To-Do List Pembangunan Coding Platform (Junior-Friendly)

Dokumen ini adalah panduan langkah demi langkah untuk membangun Coding Platform. Disusun secara bertahap, praktis, dan tidak berbelit-belit agar mudah dipelajari sambil praktek.

---

## 📌 Dokumen Referensi Desain
Sebelum memulai setiap fase, Anda dapat merujuk ke dokumen arsitektur yang sudah ada:
- [PRD.md](docs/PRD.md) — Gambaran produk, fitur MVP, dan aturan bisnis.
- [TECHNICAL-DESIGN.md](docs/TECHNICAL-DESIGN.md) — Arsitektur teknis modular monolith.
- [EXECUTION-ARCHITECTURE.md](docs/EXECUTION-ARCHITECTURE.md) — Spesifikasi sandbox runner & keamanan isolasi container.
- [DATA-MODEL.md](docs/DATA-MODEL.md) & [ERD.md](docs/ERD.md) — Skema database dan relasi antar tabel.
- [API-CONTRACT.md](docs/API-CONTRACT.md) — Format endpoint request dan response HTTP JSON.

---

## 🛠️ Fase 1: Persiapan "Alat Tempur" di Laptop
> **Tujuan:** Memastikan semua software pendukung terpasang dan dapat dijalankan dengan normal di komputer lokal.

- [ x ] **Pasang Rust toolchain**:
  - Jalankan `rustc --version` dan `cargo --version`.
- [ x ] **Pasang & Jalankan PostgreSQL**:
  - Pastikan service PostgreSQL aktif di lokal.
  - Buat 1 database kosong untuk platform ini (contoh nama: `coding_platform_dev`).
- [ x ] **Pasang Podman (atau Docker)**:
  - Jalankan perintah tes: `podman run --rm hello-world` (atau `docker run --rm hello-world`).
- [ x ] **Pasang Node.js & npm** (untuk frontend Astro):
  - Jalankan `node -v` (minimal v18/v20) dan `npm -v`.

> 🔍 **Keyword Googling untuk Belajar:**
> - *"How to install Rust with rustup on Mac"*
> - *"Install PostgreSQL local Mac Homebrew"*
> - *"Install Podman Desktop Mac"* atau *"Docker Desktop Mac setup"*

---

## 🔒 Fase 2: Eksperimen "Jantung Platform" (Sandbox Runner)
> **Tujuan:** Memahami cara mengurung kode user di dalam container aman (sandbox) langsung dari terminal sebelum kita menghubungkannya ke backend Rust.
>
> 💡 *Kenapa ini penting?* Di coding platform, kita tidak boleh percaya pada kode yang dikirim user. Kode tersebut bisa saja berisi infinite loop, mencoba memakan seluruh RAM laptop, atau mencoba mengakses internet/file rahasia.

- [ x ] **Langkah 2.1: Uji Coba Menjalankan Kode Python Sederhana di Container**
  - Jalankan perintah ini di terminal Anda:
    ```sh
    podman run --rm python:3.11-alpine python -c "print('Halo dari dalam container!')"
    ```
  - *Penjelasan perintah:*
    - `python:3.11-alpine` : Image sistem operasi Linux Alpine kecil yang sudah terpasang Python (otomatis diunduh saat pertama kali dijalankan).
    - `--rm` : Otomatis menghapus container setelah selesai jalan agar disk tidak penuh sampah.
    - `python -c "..."` : Menjalankan satu baris script Python langsung.

- [ x ] **Langkah 2.2: Pasang Batasan Sumber Daya (Resource Limits)**
  - Jalankan perintah dengan pembatas memori dan CPU:
    ```sh
    podman run --rm --memory=128m --cpus=1 python:3.11-alpine python -c "print('Kalkulasi:', 10 * 5)"
    ```
  - *Penjelasan perintah:*
    - `--memory=128m` : Membatasi penggunaan RAM maksimal 128 MB. Jika kode user mencoba mengalokasikan RAM lebih dari ini, container akan otomatis dimatikan sistem (OOM/Out of Memory).
    - `--cpus=1` : Membatasi kode hanya boleh memakai 1 inti CPU agar laptop tidak hang.

- [ x ] **Langkah 2.3: Kunci Akses Internet (Network Isolation)**
  - Uji apakah container benar-benar tidak bisa mengakses internet dengan mencoba membuka web:
    ```sh
    podman run --rm --network=none python:3.11-alpine python -c "import urllib.request; urllib.request.urlopen('https://google.com')"
    ```
  - *Hasil yang diharapkan:* Terminal akan mengeluarkan error `URLError: <urlopen error [Errno 101] Network is unreachable>`.
  - *Artinya:* Kode user terkunci sempurna, tidak bisa mengirim data keluar atau mendownload file berbahaya.

- [ x ] **Langkah 2.4: Memahami Bahaya Infinite Loop & Konsep Timeout**
  - Coba jalankan kode yang tidak akan pernah berhenti (Infinite Loop):
    ```sh
    podman run --rm --network=none python:3.11-alpine python -c "while True: pass"
    ```
  - *Perhatikan:* Terminal akan macet/diam karena kodenya berputar terus.
  - Tekan **Ctrl + C** di keyboard Anda untuk menghentikannya secara paksa.
  - *Pelajaran untuk Backend nanti:* Di backend Rust (Fase 5), kita akan memasang timer otomatis (misal maksimal 5 detik). Jika container belum selesai dalam 5 detik, backend akan langsung mematikannya secara paksa.

- [ x ] **Langkah 2.5: Baca Rangkuman Keamanan di Dokumen Repositori**
  - Buka file [EXECUTION-ARCHITECTURE.md](docs/EXECUTION-ARCHITECTURE.md) untuk melihat daftar lengkap batasan keamanan yang sudah dirancang untuk platform ini.

> 🔍 **Keyword Googling untuk Belajar Lebih Lanjut:**
> - *"Docker run memory and cpu limits"*
> - *"How online judges run untrusted code safely container"*
> - *"What is network none in docker/podman"*

---

## 🗄️ Fase 3: Rancang Database (PostgreSQL + SQLx)
> **Tujuan:** Menyiapkan struktur tabel utama di database tanpa langsung menulis logic aplikasi.

- [ x ] **Install SQLx CLI**:
  - Pasang tool migrasi dengan perintah `cargo install sqlx-cli --no-default-features --features native-tls,postgres`.
- [ x ] **Buat file migrasi tabel dasar**:
  - Konvensi: **`m_*`** untuk Master Data, **`t_*`** untuk Transaksi.
  - Tabel: `identity.m_users`, `learning.m_courses`, `learning.m_sections`, `learning.m_lessons`, `learning.m_section_lessons`, `learning.m_exercises`, `learning.m_exercise_test_cases`, `execution.t_submissions`, dan `gamification.t_xp_transactions`.
  - Lihat referensi di [DATA-MODEL.md](docs/DATA-MODEL.md) dan [REPOSITORY-STRUCTURE.md](docs/REPOSITORY-STRUCTURE.md).
- [ x ] **Jalankan migrasi lokal**:
  - Eksekusi `sqlx database reset` atau `sqlx migrate run` dan pastikan semua tabel terbuat di database lokal tanpa error.

> 🔍 **Keyword Googling untuk Belajar:**
> - *"sqlx cli install and migration tutorial rust"*
> - *"sqlx migrate add postgres example"*
> - *"PostgreSQL UUID primary key and timestamptz best practices"*

---

## 🌐 Fase 4: Backend API Dasar (Rust + Axum)
> **Tujuan:** Membuat web server Rust yang bisa menerima request HTTP dan terhubung ke database.

- [ x ] **Inisialisasi Axum Web Server**:
  - Tambahkan dependency `axum` dan `tokio` pada `api/Cargo.toml`.
  - Buat endpoint sederhana `GET /api/v1/health` yang mengembalikan `{"status": "ok"}`.
- [ x ] **Koneksikan Axum ke PostgreSQL**:
  - Buat connection pool menggunakan `sqlx::PgPool`.
  - Teruskan pool ke dalam Axum Application State.
- [ x ] **Buat Endpoint Autentikasi Dasar**:
  - Endpoint Register & Login (hashing password dengan Argon2 / bcrypt, lalu hasilkan token autentikasi).
  - Cek spesifikasi request/response di [API-CONTRACT.md](docs/API-CONTRACT.md).

> 🔍 **Keyword Googling untuk Belajar:**
> - *"Rust Axum tutorial for beginners"*
> - *"Axum state sharing with sqlx pgpool"*
> - *"Password hashing in Rust with argon2"*

---

## ⚡ Fase 5: Menghubungkan Backend ke Runner (Fitur Eksekusi Kode)
> **Tujuan:** Backend dapat menerima kode dari user, menjalankannya di dalam container, dan mengembalikan outputnya.

- [ ] **Buat fungsi pemanggil process/container di Rust**:
  - Gunakan `tokio::process::Command` untuk memanggil `podman run` (atau `docker run`).
  - Kirim source code user ke dalam container, tangkap hasil `stdout`, `stderr`, dan exit code.
  - create_submission()
  find_by_id()
  update_status()
  update_execution_result()
- [ ] **Tangani batas waktu (Timeout Handling)**:
  - Bungkus pemanggilan proses dengan `tokio::time::timeout` agar backend otomatis mematikan proses jika melebihi batas waktu (misal > 5 detik).
- [ ] **Buat Endpoint `POST /api/v1/runner/execute`**:
  - Menerima payload: `{ "language": "python", "code": "..." }`.
  - Mengembalikan output: `{ "stdout": "...", "status": "completed" }`.

> 🔍 **Keyword Googling untuk Belajar:**
> - *"Rust tokio process Command example"*
> - *"Rust tokio timeout on async command execution"*

---

## 🖥️ Fase 6: Tampilan Frontend Dasar (Astro + Editor)
> **Tujuan:** Membuat halaman belajar sederhana tempat pengguna bisa membaca materi dan mengetik kode.

- [ ] **Jalankan Frontend Astro lokal**:
  - Masuk ke direktori `web/` dan jalankan `npm run dev`.
- [ ] **Pasang Web Code Editor**:
  - Pasang komponen editor sederhana (seperti CodeMirror atau Monaco Editor) pada halaman Astro.
- [ ] **Buat Tombol "Jalankan Kode" (Run)**:
  - Ambil kode dari editor, kirim (`fetch`) ke endpoint `POST /api/v1/runner/execute`, lalu tampilkan hasilnya di output box / terminal mini.

> 🔍 **Keyword Googling untuk Belajar:**
> - *"Astro framework beginner tutorial"*
> - *"CodeMirror 6 basic setup in vanilla JS / React"*
> - *"Astro client-side fetch API example"*

---

## 🏆 Fase 7: Fitur Gamifikasi & Progres (Submit & XP)
> **Tujuan:** Menyelesaikan siklus belajar lengkap: user submit jawaban -> dinilai -> dapat XP jika berhasil.

- [ ] **Logic Evaluasi Test Case**:
  - Bandingkan output eksekusi kode user dengan expected output dari `test_cases`.
- [ ] **Append-Only XP Ledger**:
  - Jika lulus dan baru pertama kali menyelesaikan soal tersebut, simpan penambahan +100 XP ke tabel `xp_transactions`.
- [ ] **Update Tampilan User**:
  - Tampilkan total akumulasi XP dan tanda centang selesai pada materi/exercise yang telah dikerjakan.

> 🔍 **Keyword Googling untuk Belajar:**
> - *"Database transaction in sqlx rust"*
> - *"Append only ledger pattern database design"*

---

### 💡 Tips Belajar untuk Junior:
1. **Fokus satu per satu:** Jangan pindah ke Fase berikutnya sebelum Fase saat ini benar-benar selesai dan dipahami.
2. **Uji di terminal terlebih dahulu:** Sebelum memprogram sesuatu di Rust atau Frontend, biasakan mencobanya manual di CLI/terminal (misal: perintah Docker, query SQL).
3. **Dokumentasikan error:** Ketika menemui error/bug, catat pesan errornya dan cari solusinya. Ini cara tercepat berkembang dalam Rust dan Docker.
