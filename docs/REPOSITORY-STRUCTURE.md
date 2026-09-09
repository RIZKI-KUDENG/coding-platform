# Repository Structure

## 1. Overview

Project menggunakan:

* Rust
* Modular Monolith
* Domain-Driven Design (DDD)
* Clean Architecture
* PostgreSQL
* Podman untuk local container runtime
* Ephemeral container untuk code execution

Repository tidak menggunakan microservices.

Semua business modules berada dalam satu deployable backend application, sementara code runner memiliki boundary execution yang terpisah.

---

# 2. High-Level Structure

```text
project/
│
├── web/
│   └── src/
│
├── api/
│   ├── Cargo.toml
│   └── src/
│       ├── identity/
│       ├── learning/
│       ├── practice/
│       ├── execution/
│       ├── gamification/
│       ├── content_management/
│       └── shared/
│
├── runner/
│   ├── images/
│   │   ├── javascript/
│   │   └── rust/
│   │
│   └── protocol/
│
├── migrations/
│
├── tests/
│
├── docs/
│
├── .env.example
├── .gitignore
└── README.md
```

---

# 3. Architectural Boundaries

```text
┌───────────────────────────────────────────────┐
│                  API Application              │
│                                               │
│  ┌─────────────┐ ┌─────────────┐             │
│  │ Identity    │ │ Learning    │             │
│  └─────────────┘ └─────────────┘             │
│                                               │
│  ┌─────────────┐ ┌─────────────┐             │
│  │ Execution   │ │ Gamification│             │
│  └─────────────┘ └─────────────┘             │
│                                               │
└───────────────────────┬───────────────────────┘
                        │
                   Runner Port
                        │
                        ▼
               ┌─────────────────┐
               │ Code Runner     │
               │                 │
               │ Ephemeral       │
               │ Container       │
               └─────────────────┘
```

Modules dapat berkomunikasi melalui application/domain contracts.

Module tidak boleh mengakses database table module lain secara langsung.

---

# 4. Applications

```text
web/
└── src/

api/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── app.rs
    ├── config.rs
    └── router.rs
```

`api` merupakan composition root.

Tanggung jawab:

* initialize configuration
* initialize database
* initialize infrastructure
* compose modules
* configure HTTP server
* register routes
* initialize observability
* start application

Business logic tidak diletakkan di `api`.

---

# 5. Main Application

```text
api/src/main.rs
```

Tanggung jawab utama:

```text
main
 │
 ├── load configuration
 ├── initialize logging
 ├── initialize database
 ├── initialize infrastructure
 ├── initialize modules
 ├── compose dependencies
 ├── create router
 └── start HTTP server
```

`main.rs` harus tetap tipis.

Contoh conceptual flow:

```text
main()
   │
   ▼
Application::build()
   │
   ├── Identity
   ├── Learning
   ├── Execution
   └── Gamification
   │
   ▼
HTTP Server
```

---

# 6. Modules

Business capability dibagi berdasarkan bounded context di dalam crate `api`.

```text
api/src/
├── identity/
├── learning/
├── practice/
├── execution/
├── gamification/
└── content_management/
```

## Identity

Tanggung jawab:

* User
* Authentication
* Credentials
* Session/token
* Identity context

## Learning

Tanggung jawab:

* Course
* Lesson
* Exercise
* Test case
* Learning progression
* Completion rules

## Execution

Tanggung jawab:

* Submission
* Execution orchestration
* Runner interaction
* Execution result
* Resource limits

## Gamification

Tanggung jawab:

* XP
* XP transaction
* XP awarding rules

---

# 7. Module Internal Structure

Setiap module menggunakan:

```text
module/
├── domain/
├── application/
├── infrastructure/
└── presentation/
```

Example:

```text
api/src/learning/
├── domain/
├── application/
├── infrastructure/
└── presentation/
```

Layer dependency:

```text
presentation
      │
      ▼
application
      │
      ▼
domain

infrastructure ────────┐
      │                 │
      └─────────────────┘
```

Domain tidak bergantung pada infrastructure.

---

# 8. Domain Layer

Domain berisi business rules dan domain model.

Example:

```text
learning/domain/
├── entities/
│   ├── course.rs
│   ├── lesson.rs
│   ├── exercise.rs
│   └── test_case.rs
│
├── value_objects/
│   ├── course_id.rs
│   ├── lesson_id.rs
│   └── exercise_id.rs
│
├── repositories/
│   ├── course_repository.rs
│   ├── lesson_repository.rs
│   └── exercise_repository.rs
│
├── services/
│   └── progression.rs
│
└── errors.rs
```

Domain tidak boleh:

```text
use sqlx
use axum
use reqwest
use podman
```

Domain harus independent dari framework dan infrastructure.

---

# 9. Entities

Entity merepresentasikan object yang memiliki identity dan lifecycle.

Contoh:

```text
Course
Lesson
Exercise
TestCase
```

Entity tidak harus memiliki seluruh database column.

Database representation dan domain representation boleh berbeda.

---

# 10. Value Objects

Value object digunakan untuk konsep domain yang memiliki validation/semantic meaning.

Contoh:

```text
CourseId
LessonId
ExerciseId
UserId
Email
Username
Slug
XpAmount
```

Namun jangan membuat value object untuk setiap primitive hanya demi terlihat DDD.

Gunakan ketika object tersebut memiliki:

* validation
* invariant
* behavior
* semantic importance

---

# 11. Repository Interfaces

Repository interface berada di domain/application boundary.

Example:

```text
trait ExerciseRepository {
    async fn find_by_id(
        &self,
        id: ExerciseId
    ) -> Result<Option<Exercise>, RepositoryError>;
}
```

Domain tidak mengetahui implementasi database.

Implementation:

```text
api/src/learning/infrastructure/persistence/
└── postgres_exercise_repository.rs
```

---

# 12. Application Layer

Application layer mengorkestrasi use case.

Example:

```text
learning/application/
├── course/
│   ├── list_courses.rs
│   └── get_course.rs
│
├── lesson/
│   ├── get_lesson.rs
│   └── list_lessons.rs
│
├── exercise/
│   ├── get_exercise.rs
│   └── list_exercises.rs
│
└── progress/
    └── get_course_progress.rs
```

Application layer bertanggung jawab:

* use case orchestration
* transaction boundary
* authorization coordination
* repository calls
* domain service calls
* module-to-module interaction

---

# 13. Presentation Layer

Presentation menangani HTTP.

Example:

```text
learning/presentation/
└── http/
    ├── courses.rs
    ├── lessons.rs
    ├── exercises.rs
    ├── progress.rs
    ├── request.rs
    ├── response.rs
    └── routes.rs
```

Presentation layer tidak mengandung business rules.

Contoh:

```text
HTTP Request
     │
     ▼
Handler
     │
     ▼
Application Use Case
     │
     ▼
Domain
```

---

# 14. Infrastructure Layer

Infrastructure berisi implementation dari external dependencies.

Example:

```text
learning/infrastructure/
├── persistence/
│   ├── postgres_course_repository.rs
│   ├── postgres_lesson_repository.rs
│   ├── postgres_exercise_repository.rs
│   └── postgres_test_case_repository.rs
│
└── queries/
    └── ...
```

Infrastructure dapat menggunakan:

```text
SQLx
PostgreSQL
```

Domain tidak boleh mengetahui SQLx.

---

# 15. Identity Module

```text
api/src/identity/
├── domain/
│   ├── entities/
│   │   └── user.rs
│   ├── value_objects/
│   │   ├── user_id.rs
│   │   ├── email.rs
│   │   └── username.rs
│   ├── repositories/
│   │   └── user_repository.rs
│   └── errors.rs
│
├── application/
│   ├── register.rs
│   ├── login.rs
│   ├── logout.rs
│   └── get_current_user.rs
│
├── infrastructure/
│   ├── persistence/
│   │   └── postgres_user_repository.rs
│   └── authentication/
│       └── token_service.rs
│
└── presentation/
    └── http/
        ├── auth.rs
        ├── users.rs
        └── routes.rs
```

---

# 16. Learning Module

```text
api/src/learning/
├── domain/
│   ├── entities/
│   │   ├── course.rs
│   │   ├── lesson.rs
│   │   ├── exercise.rs
│   │   └── test_case.rs
│   │
│   ├── value_objects/
│   │   ├── course_id.rs
│   │   ├── lesson_id.rs
│   │   ├── exercise_id.rs
│   │   └── slug.rs
│   │
│   ├── repositories/
│   │   ├── course_repository.rs
│   │   ├── lesson_repository.rs
│   │   ├── exercise_repository.rs
│   │   └── test_case_repository.rs
│   │
│   ├── services/
│   │   └── progression.rs
│   │
│   └── errors.rs
│
├── application/
│   ├── courses/
│   ├── lessons/
│   ├── exercises/
│   └── progress/
│
├── infrastructure/
│   └── persistence/
│
└── presentation/
    └── http/
```

---

# 17. Execution Module

Execution memiliki security-sensitive infrastructure.

```text
api/src/execution/
├── domain/
│   ├── entities/
│   │   ├── submission.rs
│   │   └── execution.rs
│   │
│   ├── value_objects/
│   │   ├── submission_id.rs
│   │   ├── execution_result.rs
│   │   └── execution_limits.rs
│   │
│   ├── repositories/
│   │   └── submission_repository.rs
│   │
│   └── errors.rs
│
├── application/
│   ├── submit_solution.rs
│   ├── get_submission.rs
│   └── list_submissions.rs
│
├── infrastructure/
│   ├── persistence/
│   │   └── postgres_submission_repository.rs
│   │
│   └── runner/
│       ├── container_runner.rs
│       └── podman.rs
│
└── presentation/
    └── http/
        ├── submissions.rs
        └── routes.rs
```

---

# 18. Runner Separation

Runner implementation berada di infrastructure.

```text
Execution Application
        │
        ▼
     Runner Port
        │
        ▼
ContainerRunner
        │
        ▼
      Podman
```

Application tidak boleh mengetahui:

```text
podman command
container ID
container CLI flags
container image implementation
```

Detail tersebut hanya diketahui infrastructure.

---

# 19. Runner Directory

Repository root:

```text
runner/
├── images/
│   ├── javascript/
│   │   ├── Containerfile
│   │   ├── runner.sh
│   │   └── ...
│   │
│   └── rust/
│       ├── Containerfile
│       ├── runner.sh
│       └── ...
│
└── protocol/
    └── README.md
```

Runner image adalah deployment artifact, bukan business module.

---

# 20. Why Runner Is Outside `api`

Runner berbeda dari API application.

API:

```text
trusted application code
```

Runner:

```text
untrusted user code
```

Runner membutuhkan:

* separate container
* resource limits
* timeout
* network isolation
* filesystem isolation

Karena itu runner configuration dan image tidak sebaiknya dicampurkan dengan HTTP application.

---

# 21. Gamification Module

```text
api/src/gamification/
├── domain/
│   ├── entities/
│   │   └── xp_transaction.rs
│   │
│   ├── value_objects/
│   │   ├── xp_amount.rs
│   │   └── xp_transaction_type.rs
│   │
│   ├── repositories/
│   │   └── xp_transaction_repository.rs
│   │
│   └── errors.rs
│
├── application/
│   ├── award_xp.rs
│   └── get_xp.rs
│
├── infrastructure/
│   └── persistence/
│       └── postgres_xp_transaction_repository.rs
│
└── presentation/
    └── http/
        └── xp.rs
```

---

# 22. Module Communication

Module tidak boleh mengakses internal implementation module lain.

Bad:

```text
Learning
   │
   └── SELECT FROM gamification.xp_transactions
```

Bad:

```text
Execution
   │
   └── call GamificationRepository directly
```

Good:

```text
Execution
   │
   ▼
Application Contract
   │
   ▼
Gamification
   │
   ▼
Award XP
```

---

# 23. Domain Events

Untuk MVP, tidak perlu langsung membuat event bus infrastructure.

Namun domain event dapat digunakan sebagai conceptual boundary.

Contoh:

```text
ExercisePassed
```

Flow:

```text
Execution
   │
   ▼
ExercisePassed
   │
   ▼
Gamification
   │
   ▼
Award XP
```

Implementation awal dapat menggunakan direct application service call.

Future:

```text
Domain Event
   │
   ▼
Event Dispatcher
   │
   ├── Gamification
   ├── Analytics
   └── Notifications
```

Jangan memperkenalkan message broker hanya untuk mendapatkan event-driven architecture.

---

# 24. Shared Directory

`shared/` digunakan sangat terbatas.

```text
shared/
├── errors/
├── http/
├── database/
├── authentication/
└── types/
```

Shared hanya boleh berisi technical concerns yang benar-benar cross-cutting.

Jangan memasukkan domain logic ke `shared`.

Bad:

```text
shared/
└── learning.rs
```

Bad:

```text
shared/
└── user_service.rs
```

Business logic harus tetap berada dalam module yang memiliki ownership terhadap capability tersebut.

---

# 25. Shared HTTP

Contoh:

```text
shared/http/
├── error_response.rs
├── pagination.rs
└── response.rs
```

Digunakan untuk consistency API.

Contoh:

```text
ApiResponse<T>
ApiError
PaginationMeta
```

---

# 26. Shared Database

Database infrastructure dapat menyediakan:

```text
shared/database/
├── pool.rs
├── transaction.rs
└── error.rs
```

Tetapi query dan repository tetap dimiliki masing-masing module.

Contoh:

```text
shared/database
       │
       ▼
PostgresPool
       │
       ├── Identity Repository
       ├── Learning Repository
       ├── Execution Repository
       └── Gamification Repository
```

---

# 27. Migrations

Migration disimpan di root:

```text
migrations/
├── 0001_identity_users.sql
├── 0002_learning_courses.sql
├── 0003_learning_lessons.sql
├── 0004_learning_exercises.sql
├── 0005_learning_test_cases.sql
├── 0006_execution_submissions.sql
└── 0007_gamification_xp_transactions.sql
```

Migration naming mengikuti urutan dependency.

---

# 28. Database Schema Ownership

Database menggunakan schema per bounded module:

```text
PostgreSQL
│
├── identity
│
├── learning
│
├── execution
│
└── gamification
```

Contoh:

```text
identity.users

learning.courses
learning.lessons
learning.exercises
learning.exercise_test_cases

execution.submissions

gamification.xp_transactions
```

Module memiliki ownership terhadap schema masing-masing.

---

# 29. Cross-Schema Access

Cross-schema access harus dikontrol.

Contoh:

```text
learning
    │
    └── owns learning.*
```

Execution tidak boleh melakukan arbitrary query ke:

```text
learning.*
```

Jika membutuhkan information dari Learning:

```text
Execution Application
       │
       ▼
Learning Application Contract
```

atau read model/query interface yang disepakati.

---

# 30. Tests

Testing dibagi menjadi:

```text
tests/
├── integration/
├── api/
└── fixtures/
```

Selain itu unit test berada dekat dengan source:

```text
domain/
├── exercise.rs
└── exercise_tests.rs
```

atau menggunakan Rust module test convention.

---

# 31. Test Types

### Unit Tests

Menguji:

```text
domain rules
value objects
domain services
use case logic
```

### Integration Tests

Menguji:

```text
repository
database
module interaction
```

### API Tests

Menguji:

```text
HTTP endpoint
authentication
authorization
request validation
response contract
```

### Execution Tests

Menguji:

```text
runner
timeout
memory limit
network isolation
compile error
runtime error
test case evaluation
```

Execution security tests harus dianggap high priority.

---

# 32. Documentation

```text
docs/
├── architecture/
│   ├── technical-design.md
│   ├── data-model.md
│   ├── api-contract.md
│   ├── execution-architecture.md
│   └── repository-structure.md
│
└── development/
    └── local-setup.md
```

Dokumentasi architecture menjadi source of truth untuk architectural decisions.

---

# 33. Dependency Rules

Dependency direction:

```text
Presentation
     │
     ▼
Application
     │
     ▼
Domain
```

Infrastructure dapat implement abstraction yang dibutuhkan application/domain.

Tidak boleh:

```text
Domain → Infrastructure
Domain → HTTP
Domain → Database
```

---

# 34. Module Dependency Rules

Allowed:

```text
Identity
   ↑
Application modules

Learning
   ↑
Execution

Gamification
   ↑
Execution
```

Namun dependency harus melalui public contract.

Tidak boleh mengakses:

```text
other_module::infrastructure
other_module::presentation
other_module::internal_domain
```

Module hanya mengekspos API internal yang diperlukan.

---

# 35. Public Module Interface

Setiap module memiliki public composition interface.

Conceptual:

```text
api/src/learning/src/lib.rs
```

mengekspos:

```text
LearningModule
LearningService
LearningContracts
```

tetapi tidak mengekspos seluruh internal implementation.

Example:

```text
api/src/learning/
└── mod.rs
```

```text
pub mod application;
pub mod domain;

mod infrastructure;
mod presentation;
```

Detail internal tetap private jika tidak diperlukan oleh module lain.

---

# 36. Rust Crate

Backend MVP menggunakan satu crate Rust di `api/`. Bounded context merupakan
module internal, bukan crate terpisah. Pemisahan crate hanya dilakukan jika ada
kebutuhan dependency isolation atau build boundary yang nyata.

---

# 37. Crate Strategy

Jangan membuat setiap folder menjadi crate tanpa alasan.

Initial crate:

```text
api
```

`shared` tetap module internal dan hanya boleh berisi concern teknis lintas module.

Jika shared code masih sedikit, lebih baik jangan memaksakan shared crate terlalu awal.

---

# 38. Dependency Isolation

Contoh:

```text
identity
├── domain
├── application
├── infrastructure
└── presentation
```

Identity boleh menggunakan:

```text
sqlx
argon2
uuid
```

pada infrastructure/application sesuai kebutuhan.

Learning tidak boleh otomatis mendapatkan dependency Identity hanya karena berada dalam workspace.

Dependencies harus explicit.

---

# 39. Configuration

Configuration berada di application/infrastructure boundary.

```text
api/src/config.rs
```

Environment:

```text
DATABASE_URL
APP_HOST
APP_PORT

AUTH_SECRET

EXECUTION_TIMEOUT_SECONDS
EXECUTION_MEMORY_MB
EXECUTION_CPU_LIMIT

RUNNER_RUNTIME
RUNNER_IMAGE_PREFIX
```

Secret tidak boleh berada dalam source code atau repository.

---

# 40. Environment Files

Repository menyediakan:

```text
.env.example
```

Contoh:

```text
DATABASE_URL=
APP_HOST=0.0.0.0
APP_PORT=8080

AUTH_SECRET=

EXECUTION_TIMEOUT_SECONDS=5
EXECUTION_MEMORY_MB=128
EXECUTION_CPU_LIMIT=1

RUNNER_RUNTIME=podman
```

Actual `.env` tidak boleh di-commit.

---

# 41. Local Development

Local development:

```text
Astro
  │
  ▼
Rust API
  │
  ▼
PostgreSQL
  │
  ▼
Podman
  │
  ▼
Runner Container
```

Podman menjadi local container runtime.

Tidak ada requirement Docker Desktop.

---

# 42. Production

Production deployment belum mengunci runtime tertentu.

Architecture hanya mengharuskan:

```text
Application
     │
     ▼
Container Runtime
     │
     ▼
Ephemeral Runner
```

Production runtime dapat dipilih pada Infrastructure Design.

---

# 43. Recommended Repository

Final recommended structure:

```text
project/
│
├── web/
│   └── src/
│
├── api/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs
│       ├── app.rs
│       ├── config.rs
│       ├── router.rs
│       ├── identity/
│       ├── learning/
│       ├── practice/
│       ├── execution/
│       ├── gamification/
│       ├── content_management/
│       └── shared/
│
├── runner/
│   ├── images/
│   │   ├── javascript/
│   │   └── rust/
│   └── protocol/
│
├── migrations/
│
├── tests/
│   ├── integration/
│   ├── api/
│   └── fixtures/
│
├── docs/
│
├── .env.example
├── .gitignore
└── README.md
```

---

# 44. Architectural Principle

Repository structure mengikuti prinsip:

> **Organize code around business capabilities, then enforce architectural boundaries inside each capability.**

Bukan:

```text
controllers/
models/
services/
repositories/
```

global untuk seluruh aplikasi.

Dan bukan pula:

```text
microservice-per-module
```

untuk setiap bounded context.

Structure yang digunakan:

```text
Module
  │
  ├── Domain
  ├── Application
  ├── Infrastructure
  └── Presentation
```

dengan seluruh module berjalan sebagai satu modular monolith.

---

# 45. MVP Complexity Rule

Struktur ini memang lebih terstruktur daripada single `src/` folder, tetapi tidak berarti seluruh folder harus langsung diisi.

Folder dibuat untuk menetapkan boundary.

Implementation hanya dibuat ketika dibutuhkan.

Contoh:

```text
gamification/domain/services/
```

tidak perlu memiliki file jika belum ada domain service.

Begitu juga:

```text
shared/
```

tidak perlu dipenuhi utility random.

Tujuan structure adalah **mencegah architectural drift**, bukan membuat jumlah file sebanyak mungkin.
