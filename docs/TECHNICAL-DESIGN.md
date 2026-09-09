# Technical Design

## 1. Overview

Platform adalah coding learning platform yang menyediakan structured learning melalui:

* Course
* Lesson
* Exercise
* Code submission
* Automated test execution
* XP
* Progress
* Playground
* Practice Problems

Architecture dirancang untuk MVP dengan prinsip:

> **Simple first, scale when necessary.**

Platform tidak menggunakan microservices, Kubernetes, message broker, atau distributed worker system pada tahap awal.

---

# 2. Architecture Overview

High-level architecture:

Modular Monolith
        │
        ├── Identity
        ├── Learning
        ├── Execution
        ├── Gamification
        ├── Practice
        └── Content Management

---

# 3. Architecture Principles

## 3.1 Keep the MVP Simple

Tidak menggunakan infrastructure yang belum diperlukan.

Tidak ada:

```text
Kubernetes
Kafka
RabbitMQ
Redis Cluster
Service Mesh
Microservices
Distributed Worker Cluster
```

kecuali kebutuhan nyata muncul.

---

## 3.2 API Owns Business Logic

Frontend bertanggung jawab terhadap:

* UI
* User interaction
* Client-side state
* API communication

Backend bertanggung jawab terhadap:

* Authentication
* Authorization
* Business rules
* Submission
* Exercise completion
* XP
* Progress
* Code execution orchestration

Business logic tidak boleh bergantung pada frontend.

---

## 3.3 User Code Is Untrusted

Semua kode yang dikirim user dianggap tidak terpercaya.

Kode user tidak boleh dieksekusi langsung di Rust API process atau host system.

```text
User Code
   ↓
Untrusted
   ↓
Isolated Execution Environment
```

---
## 3.4 Domain-Driven Design

Backend menggunakan Domain-Driven Design dengan bounded context sebagai
boundary utama business domain.

Setiap bounded context memiliki:

- Domain
- Application
- Infrastructure
- Interfaces

Bounded context tidak boleh mengakses implementation detail bounded context
lain secara langsung.

---
## 3.5 Modular Monolith

Seluruh bounded context berjalan dalam satu Rust application.

Modularitas diterapkan pada level code dan database schema, bukan sebagai
microservices terpisah.

Microservices tidak digunakan pada tahap awal.

---

# 4. Technology Stack

## Architecture

- Domain-Driven Design (DDD)
- Clean Architecture
- Modular Monolith

## Frontend

```text
Astro
```

Responsibilities:

* Rendering UI
* Routing
* Client interaction
* Code editor integration
* API communication

---

## Backend

```text
Rust
```

Responsibilities:

* HTTP API
* Authentication
* Authorization
* Course/lesson/exercise data
* Submission handling
* XP processing
* Code runner orchestration

Rust dipilih untuk backend karena:

* Performance
* Strong type safety
* Low runtime overhead
* Cocok untuk execution-heavy backend
* Cocok untuk long-term scalability

---

## Database

```text
PostgreSQL
```

Responsibilities:

* User data
* Course data
* Lesson data
* Exercise data
* Test cases
* Submission history
* XP transactions

---

## Code Execution

```text
OCI-compatible container runtime
```

Podman digunakan untuk local development. Runtime production tetap berada di
belakang abstraction `ContainerRunner` dan dipilih berdasarkan deployment.

---

# 5. Repository Structure

Repository menggunakan struktur sederhana.

```text
coding-platform/
│
├── web/
│   ├── src/
│   ├── public/
│   ├── astro.config.*
│   └── package.json
│
├── api/
    └── src/
        ├── shared/
        │
        ├── identity/
        │   ├── domain/
        │   ├── application/
        │   ├── infrastructure/
        │   └── interfaces/
        │
        ├── learning/
        │   ├── domain/
        │   ├── application/
        │   ├── infrastructure/
        │   └── interfaces/
        │
        ├── practice/
        │   ├── domain/
        │   ├── application/
        │   ├── infrastructure/
        │   └── interfaces/
        │
        ├── execution/
        │   ├── domain/
        │   ├── application/
        │   ├── infrastructure/
        │   └── interfaces/
        │
        ├── gamification/
        │   ├── domain/
        │   ├── application/
        │   ├── infrastructure/
        │   └── interfaces/
        │
        └── content_management/
            ├── domain/
            ├── application/
            ├── infrastructure/
            └── interfaces/
│
├── runner/
│   ├── images/
│   ├── scripts/
│   └── ...
│
├── docs/
│   ├── PRD.md
│   ├── USER-FLOW.md
│   ├── INFORMATION-ARCHITECTURE.md
│   ├── DATA-MODEL.md
│   └── TECHNICAL-DESIGN.md
│
├── AGENTS.md
├── README.md
└── .gitignore
```

### Important

`frontend`, `backend`, dan `runner` merupakan logical separation dalam satu repository.

Tidak perlu membuat repository terpisah untuk setiap component pada tahap awal.

---

# 6. Frontend Architecture

Astro digunakan sebagai frontend application.

Struktur konseptual:

```text
frontend/
└── src/
    ├── components/
    ├── layouts/
    ├── pages/
    ├── lib/
    ├── styles/
    └── types/
```

---

## 6.1 Pages

Routing mengikuti Information Architecture.

```text
pages/
├── index
├── login
├── register
├── dashboard
├── courses
│   └── [course]
│       └── lessons
│           └── [lesson]
│               └── exercises
│                   └── [exercise]
├── playground
├── practice/problems
│   └── [problem]
└── profile
```

Implementasi routing mengikuti kemampuan routing Astro yang digunakan pada project.

---

# 7. Backend Architecture

Rust backend .

┌─────────────────────────────────────┐
│          Bounded Context            │
│                                     │
│  Interfaces                         │
│       ↓                             │
│  Application                        │
│       ↓                             │
│  Domain                             │
│       ↑                             │
│  Infrastructure                     │
└─────────────────────────────────────┘

Domain
  ↓
tidak mengetahui infrastructure

Application
  ↓
bergantung pada domain

Infrastructure
  ↓
mengimplementasikan interface domain/application

Interfaces
  ↓
berkomunikasi dengan application

---

# 8. Database Architecture

PostgreSQL menjadi source of truth untuk application data.

Core tables:

```text
PostgreSQL
├── identity
├── learning
├── execution
├── gamification
└── practice
```

contoh schema:

```text
identity
├── users
└── sessions

learning
├── courses
├── sections
├── section_lessons
├── lessons
├── exercises
└── exercise_test_cases

execution
└── submissions

gamification
└── xp_transactions

practice
├── problems
├── problem_languages
├── problem_test_cases
└── problem_submissions
```
Setiap bounded context memiliki ownership terhadap database schema-nya sendiri. Context lain tidak boleh melakukan direct write terhadap schema tersebut.
Detail schema mengikuti `DATA-MODEL.md`.

---

# 9. Authentication

Authentication ditangani oleh Rust backend.

Basic flow:

```text
Browser
   ↓
Login
   ↓
Rust API
   ↓
Validate Credentials
   ↓
Create Session
   ↓
Authenticated User
```

Authentication mechanism harus menggunakan secure session handling.

Session identifier tidak boleh disimpan sebagai plain sensitive credential di client-side storage yang mudah diakses JavaScript.

Authentication implementation harus:

* Hash password dengan password hashing algorithm yang sesuai.
* Menggunakan HTTPS pada production.
* Menggunakan secure cookies untuk session jika menggunakan cookie-based authentication.
* Regenerate session setelah authentication berhasil.
* Menyediakan logout/invalidation.

Detail implementation dapat ditentukan pada tahap API design.

---

# 10. Authorization

Authentication dan authorization dipisahkan.

Authentication:

> Siapa user ini?

Authorization:

> Apa yang boleh dilakukan user ini?

Contoh:

User hanya boleh mengakses submission miliknya sendiri.

```text
GET /submissions/:id

Check:
submission.user_id == authenticated_user.id
```

Data admin/content management dapat menggunakan authorization berbeda jika admin functionality ditambahkan.

---

# 11. Course Content Access

Course, lesson, dan exercise yang memiliki status `PUBLISHED` dapat ditampilkan kepada user.

Draft content tidak boleh ditampilkan kepada user biasa.

```text
DRAFT
  ↓
Admin / internal only

PUBLISHED
  ↓
User accessible
```

---

# 12. Lesson Access

Lesson menggunakan:

> **No Hard Locking + Recommended Progression**

Semua published lesson dapat diakses.

Backend tidak perlu membuat permission khusus untuk lesson locking.

Recommended lesson dihitung dari:

```text
First incomplete lesson
ordered by lesson.order
```

Contoh:

```text
Lesson 1 ✓
Lesson 2 ✓
Lesson 3 →
Lesson 4
Lesson 5
```

Lesson 3 menjadi recommended lesson.

---

# 13. Exercise Submission Flow

Submission flow:

```text
User
  ↓
Write Code
  ↓
Submit
  ↓
Astro
  ↓
Rust API
  ↓
Validate Request
  ↓
Load Exercise
  ↓
Load Test Cases
  ↓
Code Runner
  ↓
Execute Code
  ↓
Collect Result
  ↓
Evaluate Tests
  ↓
PASSED / FAILED
```

---

# 14. Submission Validation

Sebelum execution, backend melakukan validation.

Minimal validation:

```text
Authenticated user
Exercise exists
Exercise is published
Language is valid
Code exists
Code size within limit
```

Backend tidak boleh menerima input yang tidak dibatasi.

Contoh:

```text
Maximum code size
Maximum request body
```

Limit dapat disesuaikan berdasarkan kebutuhan MVP.

---

# 15. Code Execution Architecture

Code execution merupakan bagian paling sensitif dari platform.

Rust API **tidak mengeksekusi user code secara langsung**.

```text
Rust API
   ↓
Runner
   ↓
Runner Container
   ↓
User Code
```

Setiap execution dilakukan di environment terisolasi.

---

# 16. Code Runner

Runner bertanggung jawab untuk:

1. Menerima source code.
2. Menentukan runtime.
3. Menyiapkan execution environment.
4. Menjalankan code.
5. Memberikan input test case.
6. Mengambil output.
7. Membandingkan output dengan expected output.
8. Menghasilkan execution result.
9. Menghentikan execution jika melewati limit.
10. Membersihkan environment.

---

# 17. Container Isolation

Setiap execution dijalankan dalam container yang ephemeral.

Concept:

```text
Request
  ↓
Create Container
  ↓
Copy User Code
  ↓
Run
  ↓
Collect Output
  ↓
Destroy Container
```

Container tidak digunakan kembali untuk submission berikutnya.

Tujuannya adalah mengurangi kemungkinan state antar submission bocor.

---

# 18. Resource Limits

User code harus memiliki resource limits.

Minimal:

```text
CPU limit
Memory limit
Execution timeout
Process limit
Output size limit
Code size limit
```

Contoh konsep:

```text
CPU
  ↓
Limited

Memory
  ↓
Limited

Execution Time
  ↓
Timeout

Output
  ↓
Maximum Size
```

Nilai konkret limit ditentukan berdasarkan benchmark runtime pada tahap implementation.

Jangan menetapkan angka arbitrer sebelum mengetahui kebutuhan bahasa dan runtime.

---

# 19. Network Isolation

User code tidak membutuhkan internet untuk menjalankan exercise.

Karena itu:

```text
Container
   ↓
Network disabled
```

atau equivalent network isolation harus digunakan.

User code tidak boleh:

```text
curl external-server
wget external-file
connect to database
scan internal network
```

Execution environment tidak boleh memiliki akses network yang tidak diperlukan.

---

# 20. Filesystem Isolation

Container tidak boleh memiliki akses ke host filesystem.

User code hanya boleh mengakses filesystem ephemeral di dalam container.

Jangan melakukan:

```text
Host filesystem mount
```

untuk user-controlled execution.

Container sebaiknya menggunakan temporary writable filesystem yang akan dihancurkan setelah execution selesai.

---

# 21. Privilege Restrictions

Container execution harus menggunakan privilege minimum.

Container tidak boleh dijalankan sebagai privileged container.

Hindari:

```text
--privileged
```

dan konfigurasi yang memberikan akses berlebihan ke host.

Container harus menggunakan user non-root jika runtime memungkinkan.

---

# 22. Infinite Loop Protection

Infinite loop harus dihentikan oleh execution timeout.

Contoh:

```python
while True:
    pass
```

Flow:

```text
Execute
   ↓
Timeout exceeded
   ↓
Terminate process/container
   ↓
Return FAILED / TIMEOUT
```

Timeout harus berlaku pada level execution dan tidak hanya mengandalkan program user untuk berhenti sendiri.

---

# 23. Process Limit

User code dapat mencoba membuat banyak process/thread.

Runner harus memiliki process limit.

Tujuan:

```text
Fork bomb
Thread explosion
Process exhaustion
```

tidak menyebabkan host mengalami resource exhaustion.

---

# 24. Output Limit

User code dapat mencoba menghasilkan output dalam jumlah besar.

Contoh:

```python
while True:
    print("hello")
```

Runner harus membatasi output.

Jika output melewati limit:

```text
OUTPUT_LIMIT_EXCEEDED
```

Execution dihentikan.

---

# 25. Execution Result

Runner menghasilkan structured result.

Conceptual result:

```text
ExecutionResult
├── status
├── stdout
├── stderr
├── execution_time
└── test_results
```

Status utama:

```text
PASSED
FAILED
TIMEOUT
MEMORY_LIMIT
OUTPUT_LIMIT
RUNTIME_ERROR
```

Namun dari perspektif submission user, hasil akhirnya dapat dipetakan ke:

```text
PASSED
FAILED
```

Detail error tetap dapat digunakan untuk feedback.

---

# 26. Test Execution

Exercise dapat memiliki public dan hidden test cases.

```text
Exercise
├── Public Test Cases
└── Hidden Test Cases
```

Runner menjalankan seluruh required test cases.

Concept:

```text
Test Case 1 → PASS
Test Case 2 → PASS
Test Case 3 → PASS
Test Case 4 → PASS
             ↓
          PASSED
```

Jika salah satu required test case gagal:

```text
Test Case 1 → PASS
Test Case 2 → FAIL
Test Case 3 → PASS
             ↓
           FAILED
```

---

# 27. Test Case Comparison

Test result harus dibandingkan dengan expected output menggunakan aturan yang konsisten.

Aturan exact comparison atau normalized comparison harus ditentukan berdasarkan tipe exercise.

Untuk MVP, gunakan aturan sederhana dan konsisten.

Contoh:

```text
Expected:
Hello World

Actual:
Hello World
```

→ PASSED

Whitespace normalization dapat diterapkan jika memang diperlukan oleh exercise design.

---

# 28. Submission Lifecycle

```text
CREATED
   ↓
RUNNING
   ↓
┌───────────────┐
│               │
▼               ▼
PASSED        FAILED
```

Internal runner dapat memiliki state tambahan seperti:

```text
TIMEOUT
RUNTIME_ERROR
MEMORY_LIMIT
OUTPUT_LIMIT
```

Namun submission result yang disimpan untuk product behavior dapat tetap dipetakan ke `PASSED` atau `FAILED`, dengan execution metadata untuk debugging.
Gamification owns:
    xp_transactions
    XP award rules

Assessment:
    determines exercise result

Gamification:
    determines XP reward
    
---

# 29. XP Processing

XP diberikan setelah submission berhasil.

```text
Submission
   ↓
PASSED
   ↓
Check XP Transaction
   ↓
Already exists?
   │
 ┌─┴──┐
NO   YES
│     │
↓     ↓
+100   +0
XP
```

XP transaction harus dibuat secara idempotent.
Assessment owns:
    Submission

Learning owns:
    Exercise completion
    Lesson completion
    Course progress

---

# 30. XP Idempotency

User tidak boleh mendapatkan XP berkali-kali dari exercise yang sama.

Conceptual constraint:

```text
UNIQUE(
    user_id,
    exercise_id,
    type
)
```

Contoh:

```text
Exercise 1
  ↓
First Passed
  ↓
+100 XP

Same Exercise
  ↓
Passed again
  ↓
+0 XP
```

---

# 31. Exercise Completion

Exercise completed jika terdapat submission `PASSED`.

```text
EXISTS submission
WHERE
user_id = current_user
AND exercise_id = target_exercise
AND status = PASSED
```

Tidak diperlukan separate progress table untuk MVP.

---

# 32. Lesson Completion

Lesson completed jika seluruh exercise telah completed.

Concept:

```text
Completed Exercises
        =
Total Exercises
```

Jika:

```text
Exercise 1 ✓
Exercise 2 ✓
Exercise 3 ✓
```

maka:

```text
Lesson = COMPLETED
```

Jika salah satu belum selesai:

```text
Lesson = NOT COMPLETED
```

---

# 33. Course Progress

Course progress dihitung berdasarkan lesson completion.

```text
Completed Lessons
/
Total Lessons
```

Tidak diperlukan `course_progress` table pada MVP.

---

# 34. Recommended Lesson Calculation

Recommended lesson:

```text
First published lesson
WHERE lesson is not completed
ORDER BY lesson.order ASC
```

Jika semua lesson completed:

```text
No recommended lesson
```

---

# 35. API Architecture

Frontend berkomunikasi dengan Rust melalui HTTP API.

Conceptual structure:

```text
Astro
   ↓
HTTP
   ↓
Rust API
   ↓
Service Layer
   ↓
Repository / Runner
```

API versioning dapat menggunakan:

```text
/api/v1/
```

jika diperlukan sejak awal.

---

# 36. Initial API Surface

API awal:

```text
Authentication

POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/logout
GET  /api/v1/me
```

Courses:

```text
GET /api/v1/courses
GET /api/v1/courses/:course
```

Lessons:

```text
GET /api/v1/courses/:course/lessons/:lesson
```

Exercises:

```text
GET  /api/v1/courses/:course/lessons/:lesson/exercises/:exercise
POST /api/v1/courses/:course/lessons/:lesson/exercises/:exercise/submissions
```

Profile:

```text
GET /api/v1/me
GET /api/v1/me/xp
```

Playground:

```text
POST /api/v1/playground/run
```

Practice Problem dan Admin merupakan bagian MVP sesuai PRD. Endpoint detailnya
harus ditambahkan ke `API-CONTRACT.md` sebelum implementasi.

---

# 37. API Business Rule

API harus menjadi authority untuk:

```text
Exercise Passed
XP Award
Exercise Completion
Lesson Completion
Course Progress
```

Frontend tidak boleh menentukan:

```text
"User passed"
```

berdasarkan hasil lokal.

Frontend hanya menerima result dari backend.

---

# 38. Error Handling

Backend menggunakan structured errors.

Conceptual response:

```json
{
  "error": {
    "code": "EXERCISE_NOT_FOUND",
    "message": "Exercise not found"
  }
}
```

Error code harus stabil.

Frontend menggunakan error code untuk menentukan behavior.

---

# 39. Code Execution Errors

Execution errors dibedakan dari application errors.

Application error:

```text
EXERCISE_NOT_FOUND
UNAUTHORIZED
FORBIDDEN
INVALID_REQUEST
```

Execution error:

```text
TIMEOUT
RUNTIME_ERROR
MEMORY_LIMIT
OUTPUT_LIMIT
```

Keduanya tidak boleh tercampur.

---

# 40. Logging

Backend harus memiliki logging minimal untuk:

* HTTP request errors
* Authentication failures
* Submission failures
* Runner failures
* Unexpected application errors

Jangan menyimpan user code dalam application logs secara default.

User code dapat mengandung:

* Sensitive information
* Secrets
* Large payloads

Logging harus mempertimbangkan privacy dan storage cost.

---

# 41. Observability

MVP hanya membutuhkan observability dasar:

```text
Application logs
Error logs
Execution duration
Submission status
```

Metrics seperti:

```text
CPU usage
Memory usage
Submission throughput
Runner queue length
```

dapat ditambahkan jika traffic meningkat.

Tidak perlu membangun observability stack kompleks pada tahap awal.

---

# 42. Configuration

Configuration menggunakan environment variables.

Contoh:

```text
DATABASE_URL
SESSION_SECRET
APP_ENV
API_URL

RUNNER_IMAGE
RUNNER_TIMEOUT
RUNNER_MEMORY_LIMIT
RUNNER_CPU_LIMIT
RUNNER_OUTPUT_LIMIT
```

Secret tidak boleh disimpan di repository.

---

# 43. Environment

Minimal terdapat:

```text
Development
Production
```

Development dapat menggunakan local PostgreSQL dan local Podman runner.

Production menggunakan:

```text
Astro
Rust API
PostgreSQL
Container Runner
```

---

# 44. Deployment Architecture

Initial production deployment:

```text
                 Internet
                    │
                    ▼
             ┌──────────────┐
             │ Reverse Proxy│
             └──────┬───────┘
                    │
          ┌─────────┴─────────┐
          │                   │
          ▼                   ▼
      Astro App           Rust API
                              │
                    ┌─────────┴─────────┐
                    │                   │
                    ▼                   ▼
               PostgreSQL          Container Runner
```

Semua dapat dijalankan pada satu VM untuk MVP jika resource mencukupi.

---

# 45. Single VM Strategy

Untuk tahap awal, tidak perlu memisahkan server.

Contoh:

```text
VM
├── Reverse Proxy
├── Astro
├── Rust API
├── PostgreSQL
└── Docker
     └── Code Runner
```

Keuntungan:

* Murah
* Simple
* Mudah deployment
* Mudah debugging
* Tidak membutuhkan orchestration

Keterbatasannya adalah isolation dan resource contention antara application workload dan code execution.

Karena itu resource limit runner tetap wajib.

---

# 46. Runner Resource Protection

Walaupun VM memiliki resource besar, runner tidak boleh menggunakan resource tanpa batas.

```text
Host Resources
       │
       ├── Application
       │
       ├── Database
       │
       └── Code Runner
              │
              └── Resource Limits
```

Tujuan:

> Satu atau beberapa submission buruk tidak boleh mengganggu application dan database.

---

# 47. Concurrent Execution

MVP dapat menggunakan synchronous execution dari API ke runner selama jumlah user masih kecil.

Concept:

```text
POST submission
   ↓
Rust API
   ↓
Run container
   ↓
Wait for result
   ↓
Return response
```

Jika jumlah concurrent submission meningkat dan synchronous execution mulai menjadi bottleneck, runner dapat dipisahkan menjadi worker architecture.

Migration path:

```text
MVP

Rust API
   ↓
Container Runner


Later

Rust API
   ↓
Job Queue
   ↓
Runner Workers
   ↓
Docker
```

Queue tidak diperlukan sebelum benar-benar dibutuhkan.

---

# 48. Database Transactions

XP award dan completion update harus konsisten.

Concept:

```text
Submission PASSED
      ↓
Database Transaction
      │
      ├── Save submission
      └── Create XP transaction
      ↓
COMMIT
```

Jika transaction gagal:

```text
ROLLBACK
```

Tidak boleh terjadi:

```text
Submission = PASSED
XP = not awarded
```

karena race condition atau partial update.

---

# 49. Race Condition Protection

Submission dapat dikirim secara concurrent.

Contoh:

```text
Request A → PASSED
Request B → PASSED
```

Keduanya tidak boleh menghasilkan:

```text
+100 XP
+100 XP
```

Database unique constraint dan transaction digunakan untuk memastikan hanya satu XP reward yang berhasil dibuat.

---

# 50. Security Boundaries

Security boundary utama:

```text
Internet
   ↓
Reverse Proxy
   ↓
Rust API
   ↓
Database

Rust API
   ↓
Container Runner
   ↓
Untrusted Code
```

Untrusted code tidak boleh memiliki jalur langsung ke:

```text
Host
Database
Internal network
Application secrets
```

---

# 51. Secrets

Secrets tidak boleh masuk ke execution container.

Contoh:

```text
DATABASE_URL
SESSION_SECRET
API_KEYS
```

tidak boleh tersedia sebagai environment variable di user code container.

Runner hanya menerima data yang diperlukan untuk execution.

---

# 52. Database Access from Runner

Runner tidak memiliki database credentials.

```text
Container Runner
     X
PostgreSQL
```

Runner hanya menjalankan code dan mengembalikan result ke application.

---

# 53. API Security

API harus menerapkan:

* Authentication
* Authorization
* Request validation
* Request size limits
* Rate limiting jika diperlukan
* Secure session handling
* HTTPS production

Rate limiting dapat diterapkan terlebih dahulu pada submission endpoint jika abuse mulai terjadi.

Tidak perlu memasang distributed rate limiter untuk MVP.

---

# 54. Abuse Protection

Potential abuse:

```text
Infinite loop
Fork bomb
Memory exhaustion
Output flooding
Submission flooding
Large code payload
Network abuse
```

Protection:

```text
Timeout
CPU limit
Memory limit
Process limit
Output limit
Code size limit
Network isolation
Rate limiting
```

---

# 55. Playground vs Exercise Runner

Playground dan Exercise sama-sama membutuhkan code execution.

Namun behavior berbeda.

### Exercise

```text
Code
 ↓
Test Cases
 ↓
Passed / Failed
 ↓
XP
```

### Playground

```text
Code
 ↓
Execute
 ↓
Output
```

Playground tidak mempengaruhi exercise completion atau XP.

Pada implementation awal, keduanya dapat menggunakan runner infrastructure yang sama dengan configuration berbeda.

---

# 56. Playground Security

Playground menggunakan security restrictions yang sama dengan exercise.

User tetap tidak boleh mendapatkan:

```text
Host access
Network access
Database access
Unlimited resources
```

Playground hanya berbeda pada evaluation logic.

---

# 57. Practice Architecture

Practice Problem dibuat sebagai domain terpisah.

Concept:

```text
Problem
├── Test Cases
└── Submissions
```

Problem dapat menggunakan runner yang sama dengan exercise dan mendukung
beberapa bahasa yang dapat dipilih user.

Practice Problem tidak menggunakan lesson completion.

---

# 58. Content Management

MVP membutuhkan kemampuan admin untuk mengelola dan mem-publish content, tetapi
tidak membutuhkan CMS generik atau rules engine kompleks.

Course content dapat dikelola melalui:

* Database
* Seeder
* Internal admin tooling jika diperlukan

Content structure:

```text
Course
  ↓
Lesson
  ↓
Exercise
  ↓
Test Cases
```

Admin UI minimum merupakan bagian MVP sesuai PRD.

---

# 59. Seed Data

Development environment harus memiliki seed data untuk:

```text
Sample User
Sample Course
Sample Lessons
Sample Exercises
Sample Test Cases
```

Tujuannya agar developer dapat menjalankan seluruh learning flow tanpa membuat data secara manual.

---

# 60. Testing Strategy

Testing dibagi menjadi:

```text
Unit Tests
Integration Tests
End-to-End Tests
Runner Tests
```

### Unit

Untuk:

* Business rules
* XP calculation
* Completion logic
* Validation

### Integration

Untuk:

* API
* Database
* Submission
* Authentication

### End-to-End

Untuk core flow:

```text
Login
 ↓
Course
 ↓
Lesson
 ↓
Exercise
 ↓
Submit
 ↓
Passed
 ↓
+100 XP
```

### Runner Tests

Runner harus diuji dengan malicious/untrusted cases:

```text
Infinite loop
Large output
High memory allocation
Many processes
Runtime error
Normal execution
```

---

# 61. Development Workflow

Developer workflow:

```text
Install dependencies
      ↓
Start PostgreSQL
      ↓
Start Rust API
      ↓
Start Astro
      ↓
Start Container Runner
      ↓
Open Browser
```

Local environment harus memungkinkan developer menjalankan seluruh system secara lokal.

---

# 62. Deployment Workflow

Initial deployment:

```text
Git Push
   ↓
Build
   ↓
Run Tests
   ↓
Build Frontend
   ↓
Build Rust API
   ↓
Build Runner Images
   ↓
Run Database Migration
   ↓
Restart Services
```

CI/CD dapat ditambahkan sesuai kebutuhan.

---

# 63. Backup

PostgreSQL harus memiliki backup strategy pada production.

Minimal:

```text
Periodic PostgreSQL Backup
```

Backup harus disimpan di lokasi yang berbeda dari primary database jika memungkinkan.

---

# 64. Migration Strategy

Database schema dikelola menggunakan migration.

Contoh:

```text
migrations/
├── 001_create_users
├── 002_create_courses
├── 003_create_lessons
├── 004_create_exercises
├── 005_create_test_cases
├── 006_create_submissions
└── 007_create_xp_transactions
```

Migration harus versioned dan committed ke repository.

---

# 65. Scalability Path

MVP architecture:

```text
Single VM
   │
   ├── Astro
   ├── Rust API
   ├── PostgreSQL
   └── Container Runner
```

Jika traffic meningkat:

```text
                  Load Balancer
                       │
              ┌────────┴────────┐
              │                 │
          Rust API 1        Rust API 2
              │                 │
              └────────┬────────┘
                       │
                  PostgreSQL
                       │
                  Job Queue
                       │
              ┌────────┴────────┐
              │                 │
          Runner 1          Runner 2
```

Scaling dilakukan berdasarkan bottleneck nyata.

---

# 66. What Is Not Included in MVP

MVP tidak membutuhkan:

```text
Kubernetes
Microservices
Redis
Kafka
RabbitMQ
Distributed tracing
Service mesh
Dedicated runner cluster
Auto-scaling
Multi-region deployment
```

Fitur tersebut hanya dipertimbangkan ketika scale dan operational requirements membutuhkannya.

---

# 67. MVP Technical Scope

MVP technical scope:

```text
Frontend
├── Astro
├── Authentication UI
├── Course UI
├── Lesson UI
├── Exercise UI
├── Practice Problem UI
├── Playground UI
├── Admin UI
├── Code Editor
└── Submission Result

Backend
├── Rust
├── Authentication
├── Course API
├── Lesson API
├── Exercise API
├── Practice Problem API
├── Playground API
├── Admin Content API
├── Submission API
├── XP
├── Level
└── Progress

Database
└── PostgreSQL

Execution
└── Isolated container runner
```

---

# 68. Core System Flow

Final system flow:

```text
                       Browser
                          │
                          ▼
                       Astro
                          │
                          ▼
                      Rust API
                          │
              ┌───────────┴───────────┐
              │                       │
              ▼                       ▼
         PostgreSQL              Code Runner
                                      │
                                      ▼
                               Runner Container
                                      │
                                      ▼
                                User's Code
```

Learning flow:

```text
Course
  ↓
Lesson
  ↓
Exercise
  ↓
Write Code
  ↓
Submit
  ↓
Rust API
  ↓
Container Runner
  ↓
Run Test Cases
  ↓
PASSED / FAILED
  │
  ├── FAILED
  │      ↓
  │    Retry
  │
  └── PASSED
         ↓
      +100 XP
         ↓
   Exercise Completed
         ↓
   All Exercises Passed?
         │
         ├── No → Next Exercise
         │
         └── Yes
                ↓
         Lesson Completed
                ↓
         Course Progress
```

---

# 69. Final Technical Principles

Platform harus mengikuti prinsip berikut:

1. **Keep the architecture simple.**
2. **Rust API owns business logic.**
3. **PostgreSQL is the application source of truth.**
4. **User code is always treated as untrusted.**
5. **User code never runs directly inside the API process.**
6. **Code execution happens inside an isolated container environment.**
7. **Execution has strict CPU, memory, timeout, process, and output limits.**
8. **Runner has no access to application secrets or database credentials.**
9. **Exercise completion is derived from successful submission.**
10. **XP is awarded once per completed exercise.**
11. **All exercise completion is required for lesson completion.**
12. **Lesson access uses no hard locking.**
13. **Recommended progression guides users without restricting access.**
14. **Do not introduce infrastructure before actual scale requires it.**
15. **Scale the runner separately only when execution workload becomes a bottleneck.**

---

# 70. Technical Source of Truth

Technical implementation should follow this document together with:

```text
docs/PRD.md
docs/USER-FLOW.md
docs/INFORMATION-ARCHITECTURE.md
docs/DATA-MODEL.md
```

When implementation decisions conflict with these documents, the discrepancy should be resolved before implementing behavior that changes the product requirements.
