# Data Model

## 1. Overview

Data model platform dibangun dengan pendekatan:

* Modular Monolith
* Domain-Driven Design (DDD)
* Clean Architecture
* Database schema per module
* Store facts, derive states

Core learning relationship:

```text
Learning Path / Course
  ↓
Section
  ↓
Lesson
  ↓
Exercise
  ↓
Test Case
```

User learning activity:

```text
User
  ↓
Submission
  ↓
Exercise
```

Reward:

```text
User
  ↓
XP Transaction
  ↓
Exercise
```

Progress dan recommendation **tidak disimpan sebagai state redundant** untuk MVP. State tersebut dihitung dari data yang sudah ada.

---

# 2. Module & Database Schema

MVP menggunakan module berikut:

```text
identity
learning
execution
gamification
practice
```

Database schema:

```text
PostgreSQL
│
├── identity
│   └── m_users
│
├── learning
│   ├── m_courses
│   ├── m_sections
│   ├── m_section_lessons
│   ├── m_lessons
│   ├── m_exercises
│   └── m_exercise_test_cases
│
├── execution
│   └── t_submissions
│
├── gamification
│   └── t_xp_transactions
│
└── practice
    ├── m_problems
    ├── m_problem_languages
    ├── m_problem_test_cases
    └── t_problem_submissions
```

### Module Ownership & Naming Convention

Konvensi penamaan tabel:
- **`m_*`** = **Master Data** (Data induk / katalog konten)
- **`t_*`** = **Transaction Data** (Data aktivitas / append-only event pengguna)

| Module         | Responsibility                       |
| -------------- | ------------------------------------ |
| `identity`     | User identity dan authentication     |
| `learning`     | Learning Path/Course, section, lesson, exercise, test case |
| `execution`    | Code submission dan execution result |
| `gamification` | XP transaction dan reward            |
| `practice`     | Problem, language, test case, submission |

Module hanya memiliki dan memodifikasi data miliknya sendiri.

Cross-module relationship menggunakan identifier/reference, bukan database foreign key lintas schema.

---

# 3. Identity Module

Schema:

```text
identity
```

## 3.1 User

Table:

```text
identity.m_users
```

Fields:

| Field           | Type        | Constraint       | Description           |
| --------------- | ----------- | ---------------- | --------------------- |
| `id`            | UUID        | PK               | User identifier       |
| `email`         | VARCHAR     | UNIQUE, NOT NULL | User email            |
| `password`      | TEXT        | NOT NULL         | Password hash         |
| `username`      | VARCHAR     | UNIQUE, NOT NULL | Public username       |
| `created_at`    | TIMESTAMPTZ | NOT NULL         | Creation timestamp    |
| `updated_at`    | TIMESTAMPTZ | NOT NULL         | Last update timestamp |

User menjadi owner dari identity/authentication data.

Module lain hanya menyimpan `user_id` sebagai reference.

---

# 4. Learning Module

Schema:

```text
learning
```

Learning module memiliki:

```text
Learning Path / Course
  ↓
Section
  ↓
Lesson
  ↓
Exercise
  ↓
ExerciseTestCase
```

---

# 5. Course

Table:

```text
learning.m_courses
```

Fields:

| Field         | Type        | Constraint | Description               |
| ------------- | ----------- | ---------- | ------------------------- |
| `id`          | UUID        | PK         | Course identifier         |
| `title`       | VARCHAR     | NOT NULL   | Course name               |
| `slug`        | VARCHAR     | NOT NULL   | URL identifier            |
| `description` | TEXT        | NULL       | Course description        |
| `status`      | VARCHAR     | NOT NULL   | Course publication status |
| `created_at`  | TIMESTAMPTZ | NOT NULL   | Creation timestamp        |
| `updated_at`  | TIMESTAMPTZ | NOT NULL   | Last update timestamp     |

Constraint:

```text
UNIQUE(slug)
```

### Status

```text
DRAFT
PUBLISHED
```

Course tidak menjadi source of truth bahasa execution. Bahasa merupakan milik
Exercise sesuai PRD, dan client tidak dapat menimpanya saat submission.

---

# 5.1 Section & Reusable Lesson Membership

`learning.m_sections` mengelompokkan lesson di dalam Course/Learning Path.
`learning.m_section_lessons` menyimpan membership dan urutan lesson sehingga satu
Lesson dapat digunakan oleh lebih dari satu Learning Path tanpa menduplikasi
content.

```text
Course 1 ── * Section 1 ── * SectionLesson * ── 1 Lesson
```

Minimum fields:

```text
m_sections(id, course_id, title, order, created_at, updated_at)
m_section_lessons(section_id, lesson_id, order)
```

Constraints:

```text
UNIQUE(course_id, order)
UNIQUE(section_id, lesson_id)
UNIQUE(section_id, order)
```

---

# 6. Lesson

Table:

```text
learning.m_lessons
```

Fields:

| Field         | Type        | Constraint | Description           |
| ------------- | ----------- | ---------- | --------------------- |
| `id`          | UUID        | PK         | Lesson identifier     |
| `title`       | VARCHAR     | NOT NULL   | Lesson name           |
| `slug`        | VARCHAR     | NOT NULL   | URL identifier        |
| `description` | TEXT        | NULL       | Lesson description    |
| `status`      | VARCHAR     | NOT NULL   | Publication status    |
| `created_at`  | TIMESTAMPTZ | NOT NULL   | Creation timestamp    |
| `updated_at`  | TIMESTAMPTZ | NOT NULL   | Last update timestamp |

Constraint:

```text
UNIQUE(slug)
```

### Status

```text
DRAFT
PUBLISHED
```

---

# 7. Exercise

Table:

```text
learning.m_exercises
```

Fields:

| Field          | Type        | Constraint | Description           |
| -------------- | ----------- | ---------- | --------------------- |
| `id`           | UUID        | PK         | Exercise identifier   |
| `lesson_id`    | UUID        | NOT NULL   | Parent lesson         |
| `title`        | VARCHAR     | NOT NULL   | Exercise name         |
| `slug`         | VARCHAR     | NOT NULL   | URL identifier        |
| `description`  | TEXT        | NOT NULL   | Exercise instructions |
| `starter_code` | TEXT        | NULL       | Initial code          |
| `language`     | VARCHAR     | NOT NULL   | Execution language    |
| `order`        | INTEGER     | NOT NULL   | Exercise ordering     |
| `xp`           | INTEGER     | NOT NULL   | XP reward             |
| `status`       | VARCHAR     | NOT NULL   | Publication status    |
| `created_at`   | TIMESTAMPTZ | NOT NULL   | Creation timestamp    |
| `updated_at`   | TIMESTAMPTZ | NOT NULL   | Last update timestamp |

Relationship:

```text
Lesson 1 ────── * Exercise
```

Constraints:

```text
FOREIGN KEY(lesson_id)
REFERENCES learning.m_lessons(id)

UNIQUE(lesson_id, slug)

UNIQUE(lesson_id, order)
```

### XP

Untuk MVP:

```text
xp = 100
```

Tidak ada partial XP.

`xp` tetap disimpan pada exercise agar reward dapat dikonfigurasi per exercise tanpa mengubah struktur data ketika kebutuhan berkembang.

---

# 8. Exercise Test Case

Table:

```text
learning.m_exercise_test_cases
```

Fields:

| Field             | Type        | Constraint | Description             |
| ----------------- | ----------- | ---------- | ----------------------- |
| `id`              | UUID        | PK         | Test case identifier    |
| `exercise_id`     | UUID        | NOT NULL   | Parent exercise         |
| `input`           | TEXT        | NOT NULL   | Test input              |
| `expected_output` | TEXT        | NOT NULL   | Expected result         |
| `is_hidden`       | BOOLEAN     | NOT NULL   | Hidden/public test case |
| `order`           | INTEGER     | NOT NULL   | Execution/display order |
| `created_at`      | TIMESTAMPTZ | NOT NULL   | Creation timestamp      |
| `updated_at`      | TIMESTAMPTZ | NOT NULL   | Last update timestamp   |

Relationship:

```text
Exercise 1 ────── * ExerciseTestCase
```

Constraint:

```text
FOREIGN KEY(exercise_id)
REFERENCES learning.m_exercises(id)

UNIQUE(exercise_id, order)
```

### Visibility

```text
is_hidden = false
→ Public test case

is_hidden = true
→ Hidden test case
```

Hidden test case tidak dikirim kepada client.

---

# 9. Exercise Completion

Exercise completion tidak disimpan sebagai field:

```text
completed
```

dan tidak menggunakan:

```text
exercise_progress
```

Completion diturunkan dari submission.

```text
Submission
   │
   └── PASSED
          │
          ▼
Exercise completed for user
```

Rule:

> User dianggap telah menyelesaikan exercise apabila terdapat minimal satu submission `PASSED` untuk exercise tersebut.

Completion bersifat **per user**, bukan global pada exercise.

---

# 10. Execution Module

Schema:

```text
execution
```

Execution module bertanggung jawab terhadap:

* code submission
* execution lifecycle
* execution result

---

# 11. Submission

Table:

```text
execution.t_submissions
```

Fields:

| Field            | Type        | Constraint | Description           |
| ---------------- | ----------- | ---------- | --------------------- |
| `id`             | UUID        | PK         | Submission identifier |
| `user_id`        | UUID        | NOT NULL   | User reference        |
| `exercise_id`    | UUID        | NOT NULL   | Exercise reference    |
| `code`           | TEXT        | NOT NULL   | Submitted source code |
| `status`         | VARCHAR     | NOT NULL   | Submission result     |
| `execution_time` | INTEGER     | NULL       | Execution duration    |
| `created_at`     | TIMESTAMPTZ | NOT NULL   | Creation timestamp    |
| `updated_at`     | TIMESTAMPTZ | NOT NULL   | Last update timestamp |

### Cross-module references

```text
user_id
    → identity.m_users.id

exercise_id
    → learning.m_exercises.id
```

Reference tersebut **tidak menggunakan database foreign key lintas schema**.

Ownership tetap berada pada module masing-masing.

---

# 12. Submission Status

Untuk learning result:

```text
PASSED
FAILED
```

Namun execution lifecycle membutuhkan status internal yang lebih detail.

Karena itu execution result tidak boleh diasumsikan selalu hanya:

```text
PASSED
FAILED
```

Execution dapat menghasilkan kondisi seperti:

```text
QUEUED
RUNNING
PASSED
FAILED
TIMEOUT
COMPILE_ERROR
RUNTIME_ERROR
```

Untuk MVP, API/domain learning dapat memetakan hasil execution menjadi:

```text
PASSED
FAILED
```

Detail execution error dapat tetap menjadi concern Execution module.

Jika lifecycle asynchronous digunakan, status persistence perlu diperluas sebelum implementation.

---

# 13. Submission History

Setiap submission disimpan.

Contoh:

```text
Submission 1 → FAILED
Submission 2 → FAILED
Submission 3 → PASSED
```

Submission tidak di-overwrite.

History dapat digunakan untuk:

* debugging
* analytics
* learning history
* execution diagnostics

MVP tidak wajib menampilkan seluruh history kepada user.

---

# 14. Lesson Completion

Lesson completion diturunkan dari exercise completion.

Rule:

> Lesson dianggap completed apabila seluruh published exercise di dalam lesson telah completed oleh user.

Conceptually:

```text
completed exercises
/
published exercises
```

Lesson completed apabila:

```text
completed exercises == published exercises
```

Example:

```text
Exercise 1 ✓
Exercise 2 ✓
Exercise 3 ✓
Exercise 4 ✓

Lesson = COMPLETED
```

Jika satu saja belum selesai:

```text
Exercise 1 ✓
Exercise 2 ✓
Exercise 3 ✗
Exercise 4 ✓

Lesson = NOT COMPLETED
```

### Publishing invariant

Published lesson harus memiliki minimal satu published exercise.

---

# 15. Course Progress

Course progress tidak disimpan dalam:

```text
course_progress
```

Progress dihitung dari lesson completion.

```text
completed lessons
/
published lessons
```

Example:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 ✓
Lesson 4 ○
Lesson 5 ○

Progress = 3 / 5 = 60%
```

---

# 16. Lesson Access & Progression

Lesson tidak menggunakan hard locking.

Semua lesson dengan:

```text
status = PUBLISHED
```

dapat diakses user.

Example:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 →
Lesson 4
Lesson 5
```

User tetap dapat membuka Lesson 5 walaupun Lesson 3 belum completed.

Sistem hanya memberikan recommended progression.

---

# 17. Recommended Lesson

Recommended lesson dihitung dari published lesson dengan `order` paling kecil yang belum completed.

Example:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 →
Lesson 4 ○
Lesson 5 ○
```

Lesson 3 menjadi recommended.

Tidak ada field:

```text
is_recommended
```

Recommendation merupakan derived state.

---

# 18. Gamification Module

Schema:

```text
gamification
```

Gamification menyimpan XP sebagai transaction history.

---

# 19. XP Transaction

Table:

```text
gamification.t_xp_transactions
```

Fields:

| Field         | Type        | Constraint | Description            |
| ------------- | ----------- | ---------- | ---------------------- |
| `id`          | UUID        | PK         | Transaction identifier |
| `user_id`     | UUID        | NOT NULL   | User reference         |
| `exercise_id` | UUID        | NULL       | Exercise reference     |
| `amount`      | INTEGER     | NOT NULL   | XP amount              |
| `type`        | VARCHAR     | NOT NULL   | XP source              |
| `created_at`  | TIMESTAMPTZ | NOT NULL   | Creation timestamp     |

Cross-module references:

```text
user_id
    → identity.m_users.id

exercise_id
    → learning.m_exercises.id
```

Tidak menggunakan database FK lintas module.

---

# 20. XP Transaction Type

MVP:

```text
EXERCISE_COMPLETION
```

Future types dapat ditambahkan:

```text
DAILY_CHALLENGE
ACHIEVEMENT
```

dan sebagainya.

Untuk `EXERCISE_COMPLETION`:

```text
exercise_id
```

wajib terisi.

---

# 21. XP Award Rule

XP diberikan ketika user pertama kali menyelesaikan exercise.

Flow:

```text
Submission
   ↓
PASSED
   ↓
Check previous completion
   ↓
Already rewarded?
   │
 ┌─┴──┐
NO   YES
│     │
↓     ↓
+XP   +0
```

Example:

```text
Attempt 1 → FAILED → +0 XP
Attempt 2 → PASSED → +100 XP
Attempt 3 → PASSED → +0 XP
Attempt 4 → PASSED → +0 XP
```

Total:

```text
100 XP
```

---

# 22. XP Idempotency

Untuk `EXERCISE_COMPLETION`, user hanya boleh memperoleh reward satu kali untuk exercise yang sama.

Logical uniqueness:

```text
(user_id, exercise_id, type)
```

Untuk MVP, constraint ini dapat digunakan untuk menjamin idempotency reward.

Jika tipe XP lain memiliki semantics berbeda, constraint perlu disesuaikan berdasarkan `type`.

---

# 23. Total XP

Total XP merupakan derived value.

```text
SUM(gamification.xp_transactions.amount)
```

Tidak diperlukan:

```text
users.total_xp
```

untuk MVP.

Hal ini menghindari dua source of truth:

```text
users.total_xp
```

dan:

```text
SUM(xp_transactions.amount)
```

Jika query menjadi bottleneck, caching atau denormalization dapat ditambahkan kemudian.

---

# 24. Entity Relationship

```text
┌─────────────────────────┐
│ identity.m_users        │
└───────────┬─────────────┘
            │
            │ user_id reference
            │
      ┌─────┴───────────┐
      │                 │
      ▼                 ▼
┌──────────────────┐  ┌────────────────────────┐
│ t_submissions    │  │ t_xp_transactions      │
│ execution        │  │ gamification           │
└───────┬──────────┘  └───────────┬────────────┘
        │                         │
        │ exercise_id             │ exercise_id
        │ reference               │ reference
        └───────────┬─────────────┘
                    ▼
          ┌─────────────────────┐
          │ learning.m_exercises│
          └─────────┬───────────┘
                    │
                    ▼
          ┌──────────────────────────┐
          │ m_exercise_test_cases    │
          └──────────────────────────┘
                    ▲
                    │
          ┌─────────┴───────────┐
          │ learning.m_lessons  │
          └─────────┬───────────┘
                    │
          ┌─────────┴───────────┐
          │ learning.m_courses  │
          └─────────────────────┘
```

---

# 25. Relationship Summary

```text
identity.m_users
    ├── hasMany execution.t_submissions
    └── hasMany gamification.t_xp_transactions

learning.m_courses
    └── hasMany learning.m_sections

learning.m_sections
    └── hasMany learning.m_section_lessons

learning.m_lessons
    └── reusable through learning.m_section_lessons

learning.m_lessons
    └── hasMany learning.m_exercises

learning.m_exercises
    ├── hasMany learning.m_exercise_test_cases
    ├── hasMany execution.t_submissions
    └── hasMany gamification.t_xp_transactions

execution.t_submissions
    ├── references identity.m_users
    └── references learning.m_exercises

gamification.t_xp_transactions
    ├── references identity.m_users
    └── references learning.m_exercises
```

---

# 26. Constraints & Indexes

## Users

```text
PRIMARY KEY (id)

UNIQUE (email)

UNIQUE (username)
```

## Courses

```text
PRIMARY KEY (id)

UNIQUE (slug)

INDEX (status)
```

## Lessons

```text
PRIMARY KEY (id)

UNIQUE (slug)
```

## Sections & Memberships

```text
FOREIGN KEY sections(course_id)
UNIQUE (course_id, order)

FOREIGN KEY section_lessons(section_id)
FOREIGN KEY section_lessons(lesson_id)
UNIQUE (section_id, lesson_id)
UNIQUE (section_id, order)
```

## Exercises

```text
PRIMARY KEY (id)

FOREIGN KEY (lesson_id)

UNIQUE (lesson_id, slug)

UNIQUE (lesson_id, order)

INDEX (lesson_id)

INDEX (lesson_id, status)
```

## Exercise Test Cases

```text
PRIMARY KEY (id)

FOREIGN KEY (exercise_id)

UNIQUE (exercise_id, order)

INDEX (exercise_id)
```

## Submissions

```text
PRIMARY KEY (id)

INDEX (user_id, exercise_id)

INDEX (exercise_id, created_at)

INDEX (user_id, created_at)
```

## XP Transactions

```text
PRIMARY KEY (id)

INDEX (user_id, created_at)

INDEX (user_id, type)
```

For exercise completion reward:

```text
UNIQUE (user_id, exercise_id, type)
```

---

# 27. Data Invariants

## Course

```text
PUBLISHED course
→ memiliki minimal satu section yang dapat ditampilkan
```

## Lesson

```text
SectionLesson.lesson_id
→ harus menunjuk ke lesson dalam learning module

PUBLISHED lesson
→ memiliki minimal satu PUBLISHED exercise
```

## Exercise

```text
Exercise.lesson_id
→ harus menunjuk ke lesson dalam learning module

Exercise.xp
→ >= 0
```

## Submission

```text
Submission.user_id
→ valid user reference

Submission.exercise_id
→ valid exercise reference
```

## Exercise Completion

```text
PASSED submission
→ exercise completed for that user
```

## Lesson Completion

```text
ALL published exercises completed
→ lesson completed for that user
```

## XP

```text
First exercise completion
→ exactly one EXERCISE_COMPLETION reward
```

Repeated successful submissions:

```text
→ no additional XP
```

---

# 28. Derived State

State berikut **tidak disimpan** untuk MVP:

```text
Exercise.completed
Lesson.completed
Course.progress
Lesson.progress
RecommendedLesson
User.totalXP
```

Semua dihitung dari source of truth.

```text
Submission
    ↓
Exercise completion
    ↓
Lesson completion
    ↓
Course progress
```

Dan:

```text
XPTransaction
    ↓
Total XP
```

---

# 29. Deliberately Excluded

Entity berikut tidak digunakan sebagai source-of-truth table pada MVP:

```text
CourseProgress
LessonProgress
ExerciseProgress
Achievement
Badge
Streak
Leaderboard
Notification
Subscription
```

Level tetap merupakan fitur MVP dan dihitung dari total XP menggunakan threshold
tetap. Practice Problem juga merupakan MVP dan membutuhkan model tersendiri;
detail tabelnya harus ditetapkan sebelum migration terkait dibuat.

Entity tersebut hanya ditambahkan ketika requirement produk membutuhkannya.

---

# 30. Practice Problem Model

Practice Problem merupakan experience MVP yang terpisah dari course lesson.

Required model:

```text
Problem
ProblemTestCase
ProblemSubmission
ProblemLanguage
```

Model tersebut tidak boleh dicampur dengan Exercise karena Problem mendukung
pilihan beberapa bahasa, sedangkan Exercise memiliki satu bahasa tetap.

---

# 31. Core Data Principle

Data model mengikuti prinsip:

> **Store facts, derive states.**

Stored facts:

```text
User
Course
Section
SectionLesson
Lesson
Exercise
ExerciseTestCase
Submission
XPTransaction
```

Derived states:

```text
Exercise Completed
Lesson Completed
Course Progress
Recommended Lesson
Total XP
```

Tujuannya adalah mempertahankan satu source of truth dan menghindari sinkronisasi state redundant pada tahap awal.

---

# 32. Final Module Boundary

Final MVP boundary:

```text
┌─────────────────────────────────────────────┐
│              Modular Monolith               │
│                                             │
│  ┌────────────┐                             │
│  │ Identity   │                             │
│  │ users      │                             │
│  └────────────┘                             │
│                                             │
│  ┌─────────────────────────┐                │
│  │ Learning               │                │
│  │ courses                │                │
│  │ sections               │                │
│  │ section_lessons        │                │
│  │ lessons                │                │
│  │ exercises              │                │
│  │ test_cases             │                │
│  └─────────────────────────┘                │
│                                             │
│  ┌─────────────────────────┐                │
│  │ Execution               │                │
│  │ submissions             │                │
│  └─────────────────────────┘                │
│                                             │
│  ┌─────────────────────────┐                │
│  │ Gamification            │                │
│  │ xp_transactions         │                │
│  └─────────────────────────┘                │
│                                             │
└─────────────────────────────────────────────┘
```

Diagram di atas menunjukkan core learning path. MVP juga memiliki schema
`practice` untuk `problems`, `problem_languages`, `problem_test_cases`, dan
`problem_submissions` sesuai PRD.

Modules berkomunikasi melalui domain/application interfaces dan identifier, bukan melalui direct database ownership.

---

# 33. Final Decision

Data model MVP dianggap **final** dengan keputusan berikut:

1. Learning Path/Course → Section → Lesson → Exercise → Test Case.
2. User completion ditentukan dari successful submission.
3. Tidak ada progress table untuk MVP.
4. Lesson tidak di-hard-lock.
5. Recommended progression dihitung dari lesson order.
6. XP menggunakan transaction history.
7. XP exercise diberikan satu kali setelah first successful completion.
8. Practice Problem merupakan model MVP yang terpisah.
9. Database menggunakan schema per module.
10. Module memiliki ownership terhadap tabelnya sendiri.
11. Cross-module references tidak menggunakan foreign key lintas schema.
12. Derived state tidak disimpan sebagai source of truth.
13. Execution detail tetap menjadi responsibility Execution module.
14. Exercise menjadi sumber language/environment untuk submission exercise.
15. Published lesson harus memiliki published exercise.

Dengan keputusan ini, data model dapat digunakan sebagai dasar untuk **API Contract** dan implementasi domain/application layer.
