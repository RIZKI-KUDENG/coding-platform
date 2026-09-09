# Product Requirements Document (PRD)

## Coding Platform Indonesia

**Status:** Draft v1
**Product Type:** Programming Learning Platform
**Primary Language:** Bahasa Indonesia
**Pricing:** 100% Gratis

---

# 1. Product Overview

## 1.1 Product

Coding Platform adalah platform belajar programming berbahasa Indonesia yang membantu pengguna memahami fundamental programming melalui kombinasi:

* Learning Path
* Lesson
* Coding Exercise
* Practice Problems
* How To
* Interactive Coding Playground

Platform menggabungkan proses **belajar, praktik, dan eksplorasi coding** dalam satu tempat.

---

## 1.2 Product Vision

> **Menjadi platform belajar programming yang mudah dipahami oleh orang Indonesia.**

Platform berfokus pada pengalaman belajar programming yang sederhana, praktis, dan menggunakan Bahasa Indonesia.

---

## 1.3 Problem

Orang Indonesia yang ingin belajar programming menghadapi beberapa hambatan:

* Materi programming berbahasa Indonesia masih terbatas.
* Banyak platform programming menggunakan Bahasa Inggris.
* Platform pembelajaran interaktif tertentu relatif mahal.
* Pemula sering kesulitan menghubungkan teori dengan praktik.
* Pemula membutuhkan lingkungan untuk langsung mencoba kode.

---

## 1.4 Value Proposition

Platform memungkinkan pengguna untuk:

```text
Belajar
   ↓
Memahami konsep
   ↓
Mencoba kode
   ↓
Mengerjakan exercise
   ↓
Mendapatkan feedback
   ↓
Berlatih problem solving
```

Semua pengalaman utama tersedia dalam **Bahasa Indonesia dan gratis**.

---

# 2. Target User

Target utama:

> Orang Indonesia yang ingin belajar fundamental programming.

Fokus utama:

* Beginner.
* Orang yang belum memahami fundamental programming.
* Orang yang ingin mempelajari bahasa programming tertentu.
* Orang yang ingin meningkatkan kemampuan problem solving setelah mempelajari fundamental.

Platform tidak secara khusus ditujukan untuk advanced developer pada tahap awal.

---

# 3. Product Principles

## 3.1 Bahasa Indonesia First

Pengalaman pembelajaran utama menggunakan Bahasa Indonesia.

## 3.2 Learn by Doing

Konsep programming harus dapat langsung dipraktikkan melalui coding environment.

## 3.3 Simple

Fitur dibuat sesederhana mungkin dan tidak menggunakan logic yang tidak memberikan manfaat nyata.

## 3.4 Free

Platform dapat digunakan tanpa biaya.

## 3.5 Learning First

Platform diposisikan sebagai **learning platform dengan coding environment**, bukan sekadar online compiler.

## 3.6 Progressive Learning

User dapat mengikuti jalur pembelajaran yang terstruktur, tetapi tetap dapat mengeksplorasi content lain yang tersedia.

---

# 4. Product Structure

Platform memiliki tiga area utama:

```text
Coding Platform
│
├── Learn
│   ├── Learning Path
│   │   ├── Section
│   │   │   ├── Lesson
│   │   │   └── Exercise
│   │   └── ...
│   │
│   └── How To
│
├── Practice
│   └── Problems
│
└── Playground
    └── Free Coding Playground
```

Ketiga area memiliki tujuan berbeda.

### Learn

> Saya ingin belajar programming.

### Practice

> Saya sudah belajar dan ingin berlatih problem solving.

### Playground

> Saya ingin mencoba coding sendiri.

---

# 5. Supported Programming Languages

Target bahasa programming:

* JavaScript
* Python
* Rust
* Go
* C#
* PHP

Bahasa programming yang tersedia pada launch pertama dapat disesuaikan dengan kesiapan execution environment.

Platform tidak harus menyediakan seluruh bahasa secara bersamaan apabila execution environment belum siap.

---

# 6. Learn

## 6.1 Learning Path

Learning Path menyediakan jalur pembelajaran yang terstruktur.

Contoh:

```text
JavaScript Fundamentals

Section 1 — Basic Programming
├── Introduction to Programming
├── Variables
└── Data Types

Section 2 — Control Flow
├── Conditions
├── Loops
└── Exercise

Section 3 — Functions
├── Functions
├── Parameters
└── Return Value
```

Learning Path mengatur urutan dan hubungan content.

Lesson dan Exercise merupakan content reusable dan dapat digunakan oleh lebih dari satu Learning Path.

---

## 6.2 Section

Section digunakan untuk mengelompokkan beberapa Lesson dalam sebuah Learning Path.

Contoh:

```text
JavaScript Fundamentals
│
├── Basic Programming
│   ├── Variables
│   ├── Data Types
│   └── Operators
│
├── Control Flow
│   ├── Conditions
│   └── Loops
│
└── Functions
    ├── Functions
    └── Parameters
```

Section tidak memiliki fungsi pembelajaran khusus selain membantu struktur dan navigasi Learning Path.

---

## 6.3 Lesson

Lesson digunakan untuk menjelaskan konsep programming.

Materi dapat terdiri dari:

* Text
* Code Example
* Image
* Interactive Explanation
* Callout
* Quiz
* Exercise

Model utama:

```text
┌─────────────────────┬─────────────────────┐
│                     │                     │
│       Lesson        │     Playground      │
│                     │                     │
│   Explanation       │   const x = 10      │
│                     │                     │
│   Code Example      │   [ Run ]           │
│                     │                     │
└─────────────────────┴─────────────────────┘
```

User membaca materi di sisi kiri dan melakukan praktik coding di sisi kanan.

---

# 7. Exercise

Exercise adalah latihan coding yang merupakan bagian dari proses pembelajaran.

Exercise berbeda dengan Practice Problem.

### Exercise

Tujuan:

> Menguji pemahaman terhadap materi yang baru dipelajari.

Contoh:

```text
Lesson: Variables

Exercise:
Buat sebuah variable bernama age
dan berikan nilai 20.
```

### Exercise memiliki:

* Instruction
* Initial Code
* Programming Language
* Difficulty
* Test Cases
* XP Reward

---

## 7.1 Exercise Completion

Exercise hanya dianggap **Passed** apabila seluruh test case berhasil.

```text
10 Test Cases

10 / 10 PASS
      ↓
   PASSED
      ↓
   +100 XP
```

Jika:

```text
8 / 10 PASS
      ↓
   FAILED
      ↓
    0 XP
```

Tidak ada partial score.

---

## 7.2 Exercise Language

Satu Exercise menggunakan satu programming language.

Contoh:

```text
Exercise:
Create a function that adds two numbers.

Language:
Python
```

User tidak memilih language ketika mengerjakan Exercise.

Language ditentukan oleh Exercise.

---

# 8. Practice

Practice merupakan area untuk coding problems yang berdiri sendiri dan berorientasi pada problem solving.

Konsepnya lebih dekat dengan platform seperti LeetCode.

Tujuannya:

> Menguji kemampuan user dalam menyelesaikan programming problem.

---

## 8.1 Problem

Problem bukan bagian dari Lesson secara langsung.

Contoh:

```text
Problems

Easy
├── Two Sum
├── Reverse String
└── FizzBuzz

Medium
├── ...
└── ...

Hard
└── ...
```

Problem dapat memiliki:

* Title
* Description
* Difficulty
* Topic
* Test Cases
* Supported Languages
* Constraints
* Examples

---

## 8.2 Problem Language

Berbeda dengan Exercise, user dapat memilih programming language ketika mengerjakan Problem.

Contoh:

```text
Two Sum

Difficulty: Easy
Topic: Array

Language:
[ Python ▼ ]

[ Code Editor ]

[ Run ] [ Submit ]
```

---

# 9. How To

How To merupakan practical reference untuk membantu user menyelesaikan kebutuhan programming tertentu.

Contoh:

* How to membaca input dari user.
* How to melakukan HTTP request.
* How to membaca file.
* How to menggunakan array.
* How to melakukan sorting.
* How to membaca file di Python.

How To tidak harus menjadi bagian dari Learning Path.

How To dapat diakses secara independen.

---

# 10. Playground

Playground digunakan untuk menjalankan kode secara bebas.

User dapat:

1. Memilih programming language.
2. Menulis kode.
3. Menjalankan kode.
4. Melihat output.

Flow:

```text
Select Language
      ↓
Write Code
      ↓
Run
      ↓
Execute
      ↓
Output
```

Free Playground memiliki language selector.

---

# 11. Run vs Submit

Run dan Submit merupakan dua operasi yang berbeda.

## 11.1 Run

Run digunakan untuk mencoba kode.

```text
Code
 ↓
Run
 ↓
Execute
 ↓
Output
```

Run tidak menentukan completion.

Run juga tidak memberikan XP.

---

## 11.2 Submit

Submit digunakan untuk melakukan final submission.

```text
Code
 ↓
Submit
 ↓
Run Test Cases
 ↓
All Passed?
 ├── No  → Failed
 └── Yes → Passed
             ↓
          +100 XP
             ↓
          Progress
```

---

# 12. User & Authentication

Platform mendukung Guest User.

User tidak langsung dipaksa login ketika pertama kali membuka platform.

Guest dapat mencoba sebagian content.

Contoh:

```text
Guest
 ↓
Lesson
 ↓
Exercise
 ↓
Exercise
 ↓
Login Required
```

Login diperlukan ketika user ingin:

* Menyimpan progress.
* Melanjutkan content yang membutuhkan authentication.
* Menyimpan XP.
* Menyimpan Level.
* Menyimpan Badge.
* Menyimpan exercise completion.

Login berfungsi sebagai mekanisme persistence dan progression.

---

# 13. Progress

User yang login memiliki progress pembelajaran.

Contoh:

```text
JavaScript Fundamentals

████████░░ 80%

12 / 15 completed
```

Progress dapat menunjukkan:

* Lesson completed.
* Exercise completed.
* Learning Path progress.
* Problem progress.

Progress bersifat persistent untuk authenticated user.

---

# 14. Gamification

MVP menggunakan tiga mekanisme gamification:

* XP
* Level
* Badge

---

## 14.1 XP

Setiap content yang berhasil diselesaikan memberikan:

> **100 XP**

Tidak ada formula kompleks berdasarkan:

* Waktu.
* Jumlah test case.
* Jumlah submission.
* Jumlah percobaan.
* Performance.

Sistem XP sengaja dibuat sederhana.

---

## 14.2 Level

XP digunakan untuk menentukan Level user.

Threshold Level menggunakan aturan sederhana dan fixed.

Contoh:

```text
Level 1 → 0 XP
Level 2 → 500 XP
Level 3 → 1,000 XP
Level 4 → 1,500 XP
Level 5 → 2,000 XP
```

Nilai threshold dapat disesuaikan setelah platform memiliki cukup content dan data penggunaan.

---

## 14.3 Badge

Badge diberikan berdasarkan achievement tertentu.

Contoh:

```text
First Steps
→ Complete first Lesson

Getting Started
→ Complete 5 Lessons

First Challenge
→ Complete first Exercise

JavaScript Beginner
→ Complete JavaScript Fundamentals

Problem Solver
→ Complete 10 Problems
```

Badge menggunakan aturan sederhana.

Tidak diperlukan generic rules engine pada MVP.

---

# 15. Content Management

Content dibuat dan dikelola oleh platform owner/admin.

Admin dapat membuat dan mengelola:

* Learning Path.
* Section.
* Lesson.
* Content Block.
* Exercise.
* Test Case.
* Problem.
* How To.
* Badge.
* Programming Language configuration.

---

## 15.1 Admin Content Flow

```text
Admin
 ↓
Create Learning Path
 ↓
Create Section
 ↓
Create Lesson
 ↓
Create Exercise
 ↓
Create Test Cases
 ↓
Publish
```

Untuk Problem:

```text
Admin
 ↓
Create Problem
 ↓
Set Difficulty
 ↓
Set Topic
 ↓
Configure Test Cases
 ↓
Configure Supported Languages
 ↓
Publish
```

---

# 16. Content Structure

Lesson menggunakan pendekatan content blocks.

Contoh:

```text
Lesson: Variables

Content Blocks
│
├── Text
├── Code Example
├── Text
├── Callout
├── Code Example
└── Exercise
```

Content blocks memungkinkan admin membuat materi secara modular tanpa menyimpan seluruh Lesson sebagai satu HTML document.

Jenis content block dapat berkembang seiring kebutuhan platform.

---

# 17. Functional Requirements

## Learn

### FR-001 — Learning Path

System harus dapat menampilkan Learning Path dan urutan content di dalamnya.

### FR-002 — Section

System harus dapat mengelompokkan Lesson dalam Section.

### FR-003 — Lesson

System harus dapat menampilkan Lesson beserta content blocks.

### FR-004 — Exercise

System harus dapat menampilkan Exercise dan coding environment.

### FR-005 — Exercise Language

System harus menentukan programming language berdasarkan Exercise.

### FR-006 — Exercise Test

System harus menjalankan Exercise terhadap seluruh test case.

### FR-007 — Exercise Completion

Exercise hanya dianggap selesai apabila seluruh test case passed.

---

## Practice

### FR-008 — Problem

System harus dapat menampilkan daftar coding problems.

### FR-009 — Problem Difficulty

Problem harus memiliki difficulty.

### FR-010 — Problem Topic

Problem dapat dikategorikan berdasarkan topic.

### FR-011 — Problem Language

User dapat memilih programming language untuk mengerjakan Problem.

### FR-012 — Problem Submission

System harus dapat melakukan submission terhadap Problem.

---

## Playground

### FR-013 — Free Playground

User dapat menjalankan arbitrary code melalui Free Playground.

### FR-014 — Language Selection

Free Playground menyediakan language selector.

### FR-015 — Code Execution

System harus menjalankan source code dan mengembalikan execution result.

### FR-016 — Run

User dapat menjalankan kode tanpa melakukan submission.

---

## Authentication

### FR-017 — Guest Access

Guest dapat mencoba sebagian content.

### FR-018 — Authentication

User dapat melakukan login.

### FR-019 — Progress Persistence

System menyimpan progress user yang telah login.

---

## Gamification

### FR-020 — XP

System memberikan 100 XP ketika user berhasil menyelesaikan content yang memberikan XP.

### FR-021 — Level

System menentukan Level berdasarkan XP user.

### FR-022 — Badge

System memberikan Badge berdasarkan achievement tertentu.

---

## Content Management

### FR-023 — Admin

Admin dapat mengelola learning content.

### FR-024 — Publish

Admin dapat mem-publish content.

### FR-025 — Test Case Management

Admin dapat membuat dan mengelola test case.

---

# 18. Business Rules

### BR-001 — Exercise Completion

Exercise dianggap passed hanya jika seluruh test case passed.

### BR-002 — Partial Success

Partial test case success tidak menghasilkan completion.

### BR-003 — Failed Submission

Submission yang failed tidak memberikan XP.

### BR-004 — Successful Completion

Successful completion memberikan 100 XP.

### BR-005 — Run

Run tidak mengubah completion status.

### BR-006 — Submit

Submit dapat mengubah status Exercise/Problem menjadi Passed apabila seluruh test case berhasil.

### BR-007 — Exercise Language

Exercise memiliki satu programming language.

### BR-008 — Problem Language

Problem dapat mendukung beberapa programming language dan user dapat memilih language.

### BR-009 — Content Reusability

Lesson dan Exercise dapat digunakan pada lebih dari satu Learning Path.

### BR-010 — Guest Access

Guest memiliki akses terbatas terhadap content.

### BR-011 — Persistent Progress

Progress persistent membutuhkan authentication.

### BR-012 — XP

Completion yang memberikan XP memberikan 100 XP.

### BR-013 — Pricing

Platform tidak mengenakan biaya kepada user.

---

# 19. Non-Functional Requirements

## 19.1 Code Execution Security

Karena platform menjalankan arbitrary user code, execution environment harus memiliki resource isolation.

Minimal harus memiliki batasan:

* Execution time.
* Memory.
* CPU.
* Output size.
* Filesystem access.
* Process count.

Network access pada user code harus dibatasi atau dinonaktifkan.

---

## 19.2 Execution Safety

System harus mampu menghentikan program yang:

* Infinite loop.
* Menggunakan memory secara berlebihan.
* Menghasilkan output secara berlebihan.
* Membuat process secara berlebihan.
* Menghabiskan CPU secara berlebihan.

Contoh:

```python
while True:
    print("Hello")
```

Program tersebut tidak boleh mengganggu application server atau execution environment user lain.

Detail sandbox implementation ditentukan pada System Design.

---

# 20. MVP Scope

## 20.1 Must Have

### Learn

* Learning Path.
* Section.
* Lesson.
* Exercise.
* Test Case.
* Lesson + Playground experience.

### Practice

* Problem.
* Difficulty.
* Test Case.
* Problem submission.

### Playground

* Free Playground.
* Language selection.
* Code execution.
* Run.

### User

* Authentication.
* Guest access.
* Progress.

### Gamification

* XP.
* Level.

### Admin

* Learning Path management.
* Lesson management.
* Exercise management.
* Test Case management.
* Problem management.

---

## 20.2 Should Have

* How To.
* Badge.
* Multiple programming languages.
* Rich content blocks.
* Public test cases.
* Hidden test cases.

---

## 20.3 Out of Scope

Fitur berikut tidak termasuk MVP:

* Leaderboard.
* Social features.
* Community.
* Discussion.
* Collaboration.
* Mobile application.
* AI Tutor.
* User-generated content.
* Certificates.
* Advanced gamification.
* Advanced analytics.

---

# 21. Success Criteria

MVP dianggap berhasil apabila:

1. User Indonesia dapat belajar fundamental programming dalam Bahasa Indonesia.
2. User dapat mengikuti Learning Path.
3. User dapat membaca Lesson dan langsung mencoba kode.
4. User dapat mengerjakan Exercise.
5. System dapat melakukan automated testing.
6. Exercise hanya selesai apabila seluruh test case passed.
7. User dapat melakukan Practice Problem.
8. User dapat menggunakan Free Playground.
9. User dapat menyimpan progress setelah login.
10. User mendapatkan 100 XP setelah berhasil menyelesaikan content.
11. User dapat memperoleh Level dan Badge.
12. Admin dapat membuat dan mengelola content melalui Admin Panel.
13. Seluruh pengalaman utama tersedia secara gratis.

---

# 22. Future Scope

Setelah MVP tervalidasi, platform dapat dikembangkan dengan:

* Leaderboard.
* Community.
* Discussion.
* User-generated content.
* Collaboration.
* AI Tutor.
* Mobile application.
* Certificates.
* Advanced analytics.
* Advanced gamification.
* More programming languages.

---

# 23. Initial Domain Overview

PRD ini belum mendefinisikan database atau ERD.

Domain awal yang perlu dieksplorasi:

```text
User
│
├── Progress
├── XP
├── Level
├── Badge
└── Submission

Learn
│
├── Learning Path
│   └── Section
│       └── Lesson
│           ├── Content Block
│           └── Exercise
│               └── Test Case
│
└── How To

Practice
│
└── Problem
    └── Test Case

Playground
└── Free Execution

Programming
│
└── Language
    └── Runtime

Execution
└── Code Execution

Content Management
└── Admin
```

Struktur tersebut bukan ERD dan belum menentukan implementasi database.

---

# 24. Product Flow

## New User

```text
Landing Page
     ↓
Explore
     ↓
Learn / Practice / Playground
     ↓
Guest Access
     ↓
Login Gate
     ↓
Register / Login
     ↓
Continue Learning
```

## Learning Flow

```text
Learn
 ↓
Learning Path
 ↓
Section
 ↓
Lesson
 ↓
Read + Practice
 ↓
Exercise
 ↓
Run
 ↓
Submit
 ↓
Test Cases
 ↓
Passed
 ↓
+100 XP
 ↓
Progress Updated
```

## Practice Flow

```text
Practice
 ↓
Problems
 ↓
Select Problem
 ↓
Select Language
 ↓
Write Code
 ↓
Run
 ↓
Submit
 ↓
Test Cases
 ↓
Passed / Failed
```

## Free Playground Flow

```text
Playground
 ↓
Select Language
 ↓
Write Code
 ↓
Run
 ↓
Output
```

---

# 25. Next Step

Setelah PRD ini disepakati, proses development dilanjutkan dengan:

```text
PRD
 ↓
Domain Modeling
 ↓
Entity Identification
 ↓
Master Data
 ↓
Transactional Data
 ↓
Relationship
 ↓
ERD
 ↓
API Contract
 ↓
System Design
 ↓
Implementation
```

Database schema, API, Rust architecture, sandbox architecture, dan deployment architecture belum ditentukan dalam PRD ini.
