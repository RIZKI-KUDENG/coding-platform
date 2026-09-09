# Information Architecture

## 1. Overview

Information Architecture (IA) mendefinisikan struktur halaman, navigasi, dan hubungan antar bagian dalam coding platform.

Struktur utama platform dibagi menjadi:

```text
Public
├── Landing
├── Login
└── Register

Authenticated
├── Dashboard
├── Courses
├── Playground
├── Practice
└── Profile
```

Dalam dokumen teknis, `Course` merupakan representasi delivery dari
`Learning Path` pada PRD. Learning content memiliki struktur:

```text
Course
└── Lesson
    ├── Content
    └── Exercises
        └── Exercise
```

---

# 2. Application Structure

```text
/
├── Landing
│
├── /login
│
├── /register
│
└── Authenticated Application
    │
    ├── /dashboard
    │
    ├── /courses
    │   └── /courses/:course
    │       └── /courses/:course/lessons/:lesson
    │           └── /courses/:course/lessons/:lesson/exercises/:exercise
    │
    ├── /playground
    │
    ├── /practice
    │   └── /practice/problems/:problem
    │
    └── /profile
```

---

# 3. Public Area

Public area dapat diakses tanpa authentication.

## 3.1 Landing

Route:

```text
/
```

Purpose:

* Menjelaskan platform.
* Menjelaskan konsep belajar.
* Mengarahkan user untuk register atau login.

Primary actions:

```text
[ Get Started ]
[ Login ]
```

Landing tidak bertanggung jawab terhadap proses pembelajaran.

---

## 3.2 Login

Route:

```text
/login
```

Purpose:

* User masuk ke akun.

Primary action:

```text
[ Login ]
```

Secondary action:

```text
Don't have an account?
[ Register ]
```

Setelah login berhasil:

```text
Login
  ↓
Dashboard
```

---

## 3.3 Register

Route:

```text
/register
```

Purpose:

* Membuat akun baru.

Setelah registrasi berhasil:

```text
Register
   ↓
Dashboard
```

Untuk MVP tidak diperlukan onboarding tambahan.

---

# 4. Application Access

Halaman personal berikut membutuhkan authentication:

```text
/dashboard
/profile
```

Guest dapat menjelajahi sebagian Course/Learning Path, Lesson, Exercise,
Practice, dan Playground sesuai kebijakan content. Authentication wajib ketika
progress, XP, Level, atau submission persistent akan disimpan.

User yang belum login harus diarahkan ke:

```text
/login
```

---

# 5. Global Navigation

Authenticated application menggunakan navigation utama.

```text
┌──────────────────────────────────────────────┐
│ Logo                                         │
│                                              │
│ Dashboard   Courses   Playground   Practice│
│                                              │
│                                      Profile │
└──────────────────────────────────────────────┘
```

Navigation utama:

```text
Dashboard
Courses
Playground
Practice
Profile
```

Untuk MVP tidak perlu membuat navigation yang kompleks.

---

# 6. Dashboard

Route:

```text
/dashboard
```

Dashboard merupakan entry point utama setelah authentication.

## Responsibilities

Dashboard bertanggung jawab untuk:

* Menampilkan learning progress.
* Menampilkan course yang sedang dipelajari.
* Menampilkan lesson terakhir.
* Menampilkan total XP.
* Memberikan akses cepat ke course.

## Main Sections

```text
Dashboard
├── Continue Learning
├── XP
└── Courses
```

### Continue Learning

Menampilkan course/lesson yang paling relevan untuk dilanjutkan.

Flow:

```text
Dashboard
   ↓
Continue Learning
   ↓
Current Lesson
```

### XP

Menampilkan total XP user.

### Courses

Menampilkan course yang tersedia atau course yang sedang dipelajari.

---

# 7. Courses

Route:

```text
/courses
```

Purpose:

Menampilkan seluruh course yang tersedia.

Structure:

```text
Courses
├── Course A
├── Course B
├── Course C
└── ...
```

Setiap course dapat menampilkan:

* Nama course
* Deskripsi singkat
* Progress
* Jumlah lesson

---

# 8. Course Detail

Route:

```text
/courses/:course
```

Purpose:

Menampilkan detail sebuah course dan seluruh lesson di dalamnya.

Structure:

```text
Course Detail
├── Course Information
├── Progress
└── Lessons
    ├── Lesson 1
    ├── Lesson 2
    ├── Lesson 3
    └── ...
```

Contoh:

```text
JavaScript Fundamentals

Progress
████████░░ 40%

Lessons

✓ Variables
✓ Data Types
→ Functions
○ Arrays
○ Objects
```

Course Detail menjadi tempat utama user memilih lesson.

# 8.1 Lesson Access & Progression

Platform menggunakan pendekatan **No Hard Locking + Recommended Progression**.

Semua lesson yang sudah tersedia/published dapat diakses oleh user tanpa harus menyelesaikan lesson sebelumnya.

Contoh:

```text
JavaScript Fundamentals

✓ 01 Variables
✓ 02 Data Types
→ 03 Functions       Recommended
○ 04 Arrays
○ 05 Objects
○ 06 Loops
```

## Access Rule

User dapat membuka lesson mana pun yang tersedia:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 →
Lesson 4 ○
Lesson 5 ○
```

Tidak ada hard lock:

```text
Lesson 4 🔒
Lesson 5 🔒
```

Lesson berikutnya tetap dapat diakses meskipun lesson sebelumnya belum completed.

## Recommended Lesson

Sistem menampilkan satu lesson sebagai **Recommended** untuk membantu user mengikuti progression course.

Recommended lesson ditentukan berdasarkan progress user.

Secara umum:

```text
First incomplete lesson
        ↓
Recommended Lesson
```

Contoh:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 ✓
Lesson 4 →
Lesson 5 ○
```

Lesson 4 menjadi recommended lesson.

Jika user menyelesaikan Lesson 4:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 ✓
Lesson 4 ✓
Lesson 5 →
```

Recommendation berpindah ke Lesson 5.

## Out-of-Order Learning

User tetap dapat memilih lesson lain di luar recommended progression.

Contoh:

```text
Recommended:
Lesson 4

User memilih:
Lesson 7
```

Sistem tetap mengizinkan user membuka Lesson 7.

Tidak ada penalty terhadap progress user karena belajar secara tidak berurutan.

## Lesson Completion

Recommended progression tidak mengubah aturan completion.

Lesson tetap dianggap completed hanya jika seluruh exercise di dalamnya berhasil:

```text
Exercise 1 ✓ PASSED
Exercise 2 ✓ PASSED
Exercise 3 ✓ PASSED
Exercise 4 ✓ PASSED
        ↓
Lesson COMPLETED
```

Dengan demikian:

```text
Lesson Access
    ↓
Flexible

Lesson Completion
    ↓
Strict
```

## Design Principle

Platform memberikan **arah belajar tanpa memaksa urutan belajar**.

User baru mendapatkan progression yang jelas, sementara user yang sudah memiliki pengetahuan sebelumnya tetap dapat langsung mengakses materi yang mereka perlukan.


---

# 9. Lesson

Route:

```text
/courses/:course/lessons/:lesson
```

Lesson terdiri dari:

```text
Lesson
├── Content
└── Exercises
```

Namun secara UX, content dan exercise tidak harus menjadi dua halaman terpisah.

Untuk MVP, lesson dapat menggunakan satu learning interface:

```text
Lesson
   ↓
Content
   ↓
Exercise
   ↓
Next Exercise
```

## Responsibilities

Lesson page bertanggung jawab untuk:

* Menampilkan materi.
* Menampilkan progress exercise.
* Mengarahkan user ke exercise.
* Menampilkan status completion.

---

# 10. Exercise

Route:

```text
/courses/:course/lessons/:lesson/exercises/:exercise
```

Exercise merupakan halaman utama untuk coding.

Structure:

```text
Exercise
├── Instructions
├── Code Editor
├── Submit
└── Result
```

Contoh layout konseptual:

```text
┌──────────────────────────────────────────────┐
│ Lesson: Functions                            │
│ Exercise 3 / 5                               │
├─────────────────────┬────────────────────────┤
│                     │                        │
│ Instructions        │ Code Editor            │
│                     │                        │
│ Write a function    │ function greet(...) {  │
│ that returns...     │                        │
│                     │ }                      │
│                     │                        │
│                     │ [ Submit ]             │
├─────────────────────┴────────────────────────┤
│ Result                                       │
│ ✓ Passed                                     │
│ +100 XP                                      │
└──────────────────────────────────────────────┘
```

---

# 11. Exercise Result

Result merupakan bagian dari exercise, bukan halaman terpisah untuk MVP.

Result memiliki dua kondisi:

```text
FAILED
PASSED
```

### Failed

```text
Result
├── Failed
├── Feedback
└── Try Again
```

### Passed

```text
Result
├── Passed
├── +100 XP
└── Next Exercise
```

Jika exercise sudah pernah passed:

```text
Exercise
└── Completed
```

Submission berikutnya tidak memberikan XP tambahan.

---

# 12. Lesson Completion

Ketika seluruh exercise selesai:

```text
Exercise 1 ✓
Exercise 2 ✓
Exercise 3 ✓
Exercise 4 ✓
Exercise 5 ✓
        ↓
Lesson Completed
```

User kemudian diarahkan kembali ke course atau lesson completion state.

Contoh flow:

```text
Last Exercise
     ↓
PASSED
     ↓
All Exercises Completed
     ↓
Lesson Completed
     ↓
Continue Course
```

Tidak ada score numerik.

Status hanya:

```text
COMPLETED
NOT COMPLETED
```

---

# 13. Playground

Route:

```text
/playground
```

Playground merupakan environment untuk eksperimen coding secara bebas.

Playground tidak terikat dengan course atau lesson.

Structure:

```text
Playground
├── Language Selector
├── Code Editor
├── Run
└── Output
```

Contoh:

```text
┌──────────────────────────────────────────────┐
│ Playground                                   │
│                                              │
│ Language: [ JavaScript ▼ ]                   │
│                                              │
│ ┌──────────────────────────────────────────┐ │
│ │ console.log("Hello World")               │ │
│ │                                          │ │
│ └──────────────────────────────────────────┘ │
│                                              │
│ [ Run ]                                      │
│                                              │
│ Output                                       │
│ Hello World                                  │
└──────────────────────────────────────────────┘
```

User dapat memilih bahasa di Playground.

---

# 14. Practice

Route:

```text
/practice
```

Practice merupakan area problem-solving yang terpisah dari course.

Structure:

```text
Practice
├── Easy
├── Medium
└── Hard
```

Problem detail:

```text
/practice/problems/:problem
```

Structure:

```text
Practice Problem
├── Problem Description
├── Examples
├── Constraints
├── Code Editor
├── Submit
└── Result
```

Practice Problem menggunakan konsep submission seperti:

```text
Submit
   ↓
Run Tests
   ↓
Accepted / Failed
```

Practice tidak menjadi bagian dari lesson course.

---

# 15. Profile

Route:

```text
/profile
```

Purpose:

Menampilkan informasi pembelajaran user.

Structure:

```text
Profile
├── User Information
├── Total XP
├── Course Progress
└── Completed Lessons
```

Untuk MVP tidak diperlukan sistem profile yang kompleks.

---

# 16. Navigation Relationships

Hubungan utama antar halaman:

```text
Landing
  │
  ├── Login
  │     ↓
  │  Dashboard
  │
  └── Register
        ↓
     Dashboard
```

Authenticated:

```text
Dashboard
   │
   ├── Courses
   │     ↓
   │   Course Detail
   │     ↓
   │   Lesson
   │     ↓
   │   Exercise
   │     ↓
   │   Next Exercise
   │     ↓
   │   Lesson Completed
   │
   ├── Playground
   │
   ├── Practice
   │     ↓
   │   Problem Detail
   │
   └── Profile
```

---

# 17. Breadcrumb / Context

Learning pages harus mempertahankan konteks posisi user.

Contoh:

```text
Courses
  / JavaScript Fundamentals
  / Functions
  / Exercise 3
```

Dengan demikian user dapat mengetahui:

```text
Course
  ↓
Lesson
  ↓
Exercise
```

Context ini penting terutama ketika user berada di exercise.

---

# 18. Back Navigation

User harus dapat kembali ke level sebelumnya.

```text
Exercise
   ↑
Lesson
   ↑
Course
   ↑
Courses
```

Navigasi kembali tidak boleh menghapus progress yang sudah tersimpan.

Contoh:

```text
Exercise 3 PASSED
     ↓
User kembali ke Lesson
     ↓
Exercise 3 tetap COMPLETED
```

---

# 19. Access Rules

## Public

```text
/
 /login
 /register
```

Dapat diakses tanpa authentication.

## Authenticated or Guest-Limited

```text
/dashboard
/courses
/courses/:course
/courses/:course/lessons/:lesson
/courses/:course/lessons/:lesson/exercises/:exercise
/playground
/practice
/practice/problems/:problem
/profile
```

Route content dapat dibuka oleh guest jika content policy mengizinkan.
Dashboard, Profile, progress persistent, XP/Level, dan submission tersimpan
tetap membutuhkan authentication.

---

# 20. URL Structure

URL structure menggunakan hierarchy yang merepresentasikan hubungan data.

```text
/courses
/courses/:course
/courses/:course/lessons/:lesson
/courses/:course/lessons/:lesson/exercises/:exercise
```

Untuk Practice Problem:

```text
/practice
/practice/problems/:problem
```

Untuk area independen:

```text
/playground
/profile
```

---

# 21. MVP Information Architecture

MVP hanya membutuhkan struktur berikut:

```text
/
├── /login
├── /register
│
└── Authenticated
    ├── /dashboard
    │
    ├── /courses
    │   └── /courses/:course
    │       └── /courses/:course/lessons/:lesson
    │           └── /courses/:course/lessons/:lesson/exercises/:exercise
    │
    ├── /playground
    │
    ├── /practice
    │   └── /practice/problems/:problem
    │
    └── /profile
```

---

# 22. Design Principles

Information Architecture mengikuti prinsip berikut:

### 1. Learning first

Course → Lesson → Exercise merupakan struktur utama platform.

### 2. Simple navigation

Jumlah item navigasi utama dibatasi agar platform tidak terasa seperti dashboard enterprise.

### 3. Clear context

User selalu dapat mengetahui course, lesson, dan exercise yang sedang dikerjakan.

### 4. Separation of learning modes

Lesson-based learning, Playground, dan Practice merupakan tiga pengalaman berbeda.

```text
Courses
   ↓
Structured Learning

Playground
   ↓
Free Coding

Practice
   ↓
Problem Solving
```

### 5. Progress is persistent

Progress exercise dan lesson tetap tersimpan ketika user berpindah halaman.

### 6. No unnecessary hierarchy

Jangan membuat nested navigation atau halaman tambahan jika informasi tersebut dapat ditampilkan pada halaman yang sudah ada.

---

# 23. Core Information Hierarchy

Struktur informasi paling penting dalam platform:

```text
Course
  │
  └── Lesson
        │
        ├── Content
        │
        └── Exercise
              │
              └── Submission
```

Sedangkan user memiliki:

```text
User
  │
  ├── Progress
  │    ├── Course Progress
  │    ├── Lesson Progress
  │    └── Exercise Progress
  │
  └── XP
```

Struktur ini menjadi dasar untuk tahap berikutnya, yaitu **Data Model**.
