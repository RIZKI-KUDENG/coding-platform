# User Flow

## 1. Overview

Platform menggunakan model pembelajaran berbasis **lesson dan exercise**.

Core learning loop:

```text
Learn
  ↓
Code
  ↓
Submit
  ↓
Passed / Failed
  ↓
If Passed → +100 XP → Next Exercise
  ↓
All Exercises Passed
  ↓
Lesson Completed
```

Sistem menggunakan konsep **binary completion**.

Tidak terdapat sistem nilai parsial. Sebuah exercise hanya memiliki dua kondisi hasil:

* `PASSED`
* `FAILED`

Jika exercise berhasil, user mendapatkan **100 XP**.

---

# 2. High-Level Product Flow

```text
Landing
   ↓
Login / Register
   ↓
Dashboard
   │
   ├── Courses
   │      ↓
   │    Course
   │      ↓
   │    Lesson
   │      ↓
   │    Lesson Content
   │      ↓
   │    Exercise
   │      ↓
   │    Submit
   │      ↓
   │   ┌───────────────┐
   │   │ Passed / Failed│
   │   └───────┬───────┘
   │           │
   │      ┌────┴────┐
   │      │         │
   │    Failed    Passed
   │      │         │
   │    Retry     +100 XP
   │                │
   │                ↓
   │          Next Exercise
   │                │
   │         All Passed?
   │                │
   │                ↓
   │        Lesson Completed
   │
   ├── Playground
   │
   ├── Practice
   │
   └── Profile
```

---

# 3. Authentication Flow

User dapat membuat akun atau masuk ke akun yang sudah ada.

```text
Landing
   │
   ├── Register
   │      ↓
   │   Create Account
   │      ↓
   │   Dashboard
   │
   └── Login
          ↓
       Dashboard
```

Untuk MVP, authentication dibuat sederhana tanpa onboarding yang panjang.

---

# 4. Dashboard Flow

Dashboard merupakan titik utama user untuk melanjutkan pembelajaran.

```text
Dashboard
   │
   ├── Continue Learning
   │      ↓
   │    Current Course / Lesson
   │
   ├── Browse Courses
   │      ↓
   │    Course List
   │      ↓
   │    Course Detail
   │
   ├── Playground
   │
   ├── Practice
   │
   └── Profile
```

Dashboard dapat menampilkan:

* Course yang sedang dipelajari
* Lesson terakhir
* Progress pembelajaran
* Total XP
* Course yang tersedia

---

# 5. Course Flow

```text
Courses
   ↓
Select Course
   ↓
Course Detail
   ↓
Lesson List
```

Course terdiri dari beberapa lesson.

```text
Course
├── Lesson 1
├── Lesson 2
├── Lesson 3
├── Lesson 4
└── ...
```

Setiap lesson memiliki materi dan exercise.

---

# 6. Lesson Flow

Lesson terdiri dari:

```text
Lesson
   ↓
Lesson Content
   ↓
Exercise 1
   ↓
Exercise 2
   ↓
Exercise 3
   ↓
...
   ↓
Exercise N
   ↓
Lesson Completed
```

Lesson content dapat berisi:

* Penjelasan konsep
* Contoh kode
* Informasi tambahan
* Instruksi exercise

Setelah user selesai membaca materi, user dapat mengerjakan exercise.

---

# 7. Exercise Flow

Exercise merupakan unit utama untuk menguji pemahaman user.

```text
Exercise
   ↓
Read Instructions
   ↓
Write Code
   ↓
Submit
   ↓
Execute Code
   ↓
Run Tests
   ↓
Passed?
```

Hasil submission hanya memiliki dua kondisi:

```text
PASSED
FAILED
```

---

# 8. Failed Exercise

Jika kode user tidak memenuhi test case:

```text
Submit
   ↓
Run Tests
   ↓
FAILED
   ↓
Show Feedback
   ↓
Retry
```

User dapat memperbaiki kode dan melakukan submission kembali.

Contoh:

```text
┌──────────────────────────────┐
│        Exercise Failed      │
│                              │
│        Tests Failed         │
│                              │
│  Your solution did not pass │
│  all required tests.        │
│                              │
│        [ Try Again ]        │
└──────────────────────────────┘
```

User **tidak mendapatkan XP** dari submission yang gagal.

XP hanya diberikan ketika exercise berhasil diselesaikan.

---

# 9. Passed Exercise

Jika seluruh test case berhasil:

```text
Submit
   ↓
Run Tests
   ↓
PASSED
   ↓
+100 XP
   ↓
Exercise Completed
   ↓
Next Exercise
```

Contoh:

```text
┌──────────────────────────────┐
│        Exercise Passed      │
│                              │
│       All Tests Passed      │
│                              │
│          +100 XP             │
│                              │
│      [ Next Exercise ]      │
└──────────────────────────────┘
```

Setiap exercise yang berhasil diselesaikan memberikan:

```text
XP = 100
```

Tidak ada XP parsial.

---

# 10. Exercise Completion Rule

Exercise dianggap completed hanya jika submission terakhir berhasil.

```text
FAILED
  ↓
Not Completed

PASSED
  ↓
Completed
  ↓
+100 XP
```

Jika user gagal berkali-kali kemudian berhasil:

```text
Attempt 1 → FAILED
Attempt 2 → FAILED
Attempt 3 → FAILED
Attempt 4 → PASSED
                    ↓
                 +100 XP
```

XP diberikan ketika exercise pertama kali berhasil diselesaikan.

Submission berikutnya terhadap exercise yang sudah completed **tidak memberikan XP tambahan**.

---

# 11. Lesson Completion

Lesson hanya dianggap completed apabila **semua exercise di dalam lesson sudah passed**.

Contoh:

```text
Exercise 1 ✓ PASSED
Exercise 2 ✓ PASSED
Exercise 3 ✓ PASSED
Exercise 4 ✓ PASSED
Exercise 5 ✓ PASSED
            │
            ↓
     Lesson Completed
```

Jika masih ada satu exercise yang belum passed:

```text
Exercise 1 ✓
Exercise 2 ✓
Exercise 3 ✗
Exercise 4 ✓
Exercise 5 ✓

Lesson = NOT COMPLETED
```

User harus menyelesaikan seluruh exercise.

---

# 12. Lesson Completion Result

Setelah seluruh exercise berhasil:

```text
All Exercises Passed
        ↓
Lesson Completed
        ↓
Update Lesson Progress
        ↓
Continue Course
```

Tidak ada score tambahan.

Lesson tidak memiliki nilai seperti:

```text
20
50
75
80
92
```

Status lesson hanya:

```text
COMPLETED
NOT COMPLETED
```

XP berasal dari exercise yang berhasil diselesaikan.

---

# 13. XP System

XP menggunakan aturan sederhana.

### Exercise Passed

```text
Exercise PASSED
      ↓
   +100 XP
```

### Exercise Failed

```text
Exercise FAILED
      ↓
    +0 XP
```

### Lesson Completed

Lesson completion **tidak memberikan XP tambahan secara otomatis**.

XP berasal dari setiap exercise yang berhasil.

Contoh lesson dengan 5 exercise:

```text
Exercise 1 → +100 XP
Exercise 2 → +100 XP
Exercise 3 → +100 XP
Exercise 4 → +100 XP
Exercise 5 → +100 XP
──────────────────────
Total      → 500 XP
```

Jika user baru menyelesaikan 3 exercise:

```text
Exercise 1 → +100 XP
Exercise 2 → +100 XP
Exercise 3 → +100 XP
Exercise 4 → FAILED
Exercise 5 → NOT ATTEMPTED

Total XP = 300
Lesson = NOT COMPLETED
```

---

# 14. Duplicate XP Prevention

XP tidak boleh diberikan berkali-kali untuk exercise yang sama.

Contoh:

```text
Exercise
   ↓
First PASSED
   ↓
+100 XP
```

Jika user membuka kembali exercise tersebut:

```text
Exercise already PASSED
   ↓
No additional XP
```

Dengan demikian:

```text
Exercise A
  First Passed  → +100 XP
  Re-submit     → +0 XP
  Re-submit     → +0 XP
  Re-submit     → +0 XP
```

XP merupakan reward untuk **completion**, bukan untuk jumlah submission.

---

# 15. Course Progress

Progress course ditentukan berdasarkan lesson yang sudah completed.

Contoh:

```text
Course
├── Lesson 1 ✓
├── Lesson 2 ✓
├── Lesson 3 →
├── Lesson 4
└── Lesson 5
```

Jika course memiliki 5 lesson dan 2 sudah completed:

```text
Progress = 2 / 5
```

Progress dapat ditampilkan sebagai:

```text
████████░░░░░░░░░░ 40%
```

Progress course bukan score.

---

# 16. Playground Flow

Playground merupakan area terpisah dari lesson.

User dapat memilih bahasa pemrograman di Playground.

```text
Dashboard
   ↓
Playground
   ↓
Select Language
   ↓
Code Editor
   ↓
Run
   ↓
Execution Result
```

Contoh:

```text
Playground

Language:
[ JavaScript ▼ ]

┌──────────────────────────────┐
│ function hello() {           │
│   console.log("Hello");      │
│ }                            │
└──────────────────────────────┘

[ Run ]

Output:
Hello
```

Pemilihan bahasa berlaku di Playground.

---

# 17. Language Selection Rule

Pada lesson, user **tidak memilih bahasa untuk setiap exercise**.

Bahasa ditentukan oleh exercise, sesuai PRD. Sebuah Learning Path/Course dapat
berisi exercise dengan bahasa berbeda apabila materi memang membutuhkannya.

Contoh:

```text
Exercise
   ↓
language = javascript
   ↓
Runner JavaScript
```

User tidak perlu memilih:

```text
[ JavaScript ▼ ]
[ Python ▼ ]
[ Rust ▼ ]
```

pada setiap exercise.

Sedangkan Playground bersifat bebas:

```text
Playground
   ↓
User selects language
```

---

# 18. Practice Problem Flow

Practice Problem merupakan section terpisah dari lesson-based learning.

Practice Problem dapat digunakan untuk problem solving yang lebih kompleks, seperti model platform coding challenge.

```text
Dashboard
   ↓
Practice
   ↓
Problem List
   ↓
Select Problem
   ↓
Problem Description
   ↓
Write Code
   ↓
Submit
   ↓
Run Tests
   ↓
Accepted / Failed
```

Contoh kategori:

```text
Practice
├── Easy
├── Medium
└── Hard
```

Practice Problem tidak menjadi bagian dari lesson fundamental.

---

# 19. Profile Flow

Profile menampilkan informasi pembelajaran user.

```text
Profile
   │
   ├── Total XP
   ├── Courses Progress
   ├── Completed Lessons
   └── Statistics
```

Untuk MVP, profile dibuat sederhana.

---

# 20. Core State Model

### Exercise

Exercise hanya membutuhkan state utama:

```text
NOT_COMPLETED
COMPLETED
```

Submission memiliki hasil:

```text
FAILED
PASSED
```

Hubungannya:

```text
Submission PASSED
        ↓
Exercise COMPLETED
        ↓
+100 XP
```

---

### Lesson

Lesson:

```text
NOT_COMPLETED
COMPLETED
```

Rule:

```text
ALL EXERCISES COMPLETED
        ↓
LESSON COMPLETED
```

---

### Course

Course progress dihitung dari lesson yang completed.

```text
Completed Lessons
        /
Total Lessons
```

---

# 21. Core Learning Loop

Core loop platform adalah:

```text
┌───────────────┐
│     Learn     │
└───────┬───────┘
        ↓
┌───────────────┐
│      Code     │
└───────┬───────┘
        ↓
┌───────────────┐
│     Submit    │
└───────┬───────┘
        ↓
   ┌────┴────┐
   │         │
 FAILED    PASSED
   │         │
   ↓         ↓
 Retry     +100 XP
             │
             ↓
       Next Exercise
             │
             ↓
       All Passed?
             │
             ↓
      Lesson Completed
             │
             ↓
       Continue Course
```

Prinsip utama:

> **Learn → Code → Submit → Pass → +100 XP → Continue**

Platform tidak menggunakan sistem grading parsial untuk exercise maupun lesson. Completion bersifat binary.
