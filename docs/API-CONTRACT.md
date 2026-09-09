# API Contract

## 1. Overview

API digunakan sebagai kontrak komunikasi antara frontend dan backend platform.

Architecture:

```text
Frontend
   │
   │ HTTP / JSON
   ▼
REST API
   │
   ▼
Modular Monolith
   │
   ├── Identity
   ├── Learning
   ├── Execution
   └── Gamification
```

API menggunakan REST dengan JSON sebagai format request dan response.

Base URL:

```text
/api/v1
```

---

# 2. API Design Principles

API mengikuti prinsip:

* RESTful resource-oriented API
* JSON request/response
* Versioned API
* Stateless request
* Authentication melalui access token/session mechanism
* Consistent error response
* Derived state tidak dibuat sebagai resource tersendiri
* Client tidak mengakses database secara langsung

API tidak mengekspos internal module structure secara mentah.

Contoh:

```text
GOOD
GET /api/v1/courses/{course_id}

AVOID
GET /api/v1/learning/courses/{course_id}
```

Module merupakan architectural boundary internal, bukan public API structure.

---

# 3. Authentication

Authentication berada di Identity module.

## 3.1 Register

```http
POST /api/v1/auth/register
Content-Type: application/json
```

Request:

```json
{
  "email": "user@example.com",
  "username": "rizki",
  "password": "secure-password"
}
```

Response:

```http
201 Created
```

```json
{
  "data": {
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "rizki"
    },
    "access_token": "token"
  }
}
```

---

# 4. Login

```http
POST /api/v1/auth/login
Content-Type: application/json
```

Request:

```json
{
  "email": "user@example.com",
  "password": "secure-password"
}
```

Response:

```http
200 OK
```

```json
{
  "data": {
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "rizki"
    },
    "access_token": "token"
  }
}
```

---

# 5. Logout

```http
POST /api/v1/auth/logout
Authorization: Bearer <token>
```

Response:

```http
204 No Content
```

---

# 6. Current User

```http
GET /api/v1/me
Authorization: Bearer <token>
```

Response:

```http
200 OK
```

```json
{
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "rizki"
  }
}
```

---

# 7. Course API

Course discovery merupakan public/read operation.

---

## 7.1 List Courses

```http
GET /api/v1/courses
```

Query parameters:

```text
?page=1
&limit=20
&status=published
```

Response:

```http
200 OK
```

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "JavaScript Fundamentals",
      "slug": "javascript-fundamentals",
      "description": "Learn the fundamentals of JavaScript."
    }
  ],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 1
  }
}
```

Default behavior:

```text
status = published
```

Client tidak perlu meminta draft course.

---

# 8. Get Course

```http
GET /api/v1/courses/{course_id}
```

Response:

```http
200 OK
```

```json
{
  "data": {
    "id": "uuid",
    "title": "JavaScript Fundamentals",
    "slug": "javascript-fundamentals",
    "description": "Learn the fundamentals of JavaScript."
  }
}
```

---

# 9. Get Course by Slug

Untuk URL yang user-facing, slug lebih cocok digunakan daripada UUID.

```http
GET /api/v1/courses/slug/{slug}
```

Example:

```http
GET /api/v1/courses/slug/javascript-fundamentals
```

Response:

```json
{
  "data": {
    "id": "uuid",
    "title": "JavaScript Fundamentals",
    "slug": "javascript-fundamentals",
    "description": "Learn the fundamentals of JavaScript."
  }
}
```

---

# 10. Course Lessons

```http
GET /api/v1/courses/{course_id}/lessons
Authorization: Bearer <token>
```

Response:

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "Variables",
      "slug": "variables",
      "description": "Learn variables.",
      "order": 1,
      "status": "published",
      "completed": true
    },
    {
      "id": "uuid",
      "title": "Functions",
      "slug": "functions",
      "description": "Learn functions.",
      "order": 2,
      "status": "published",
      "completed": false
    }
  ]
}
```

`completed` merupakan **derived state**.

Tidak terdapat database field `lessons.completed`.

---

# 11. Course Progress

Progress tidak menjadi resource database tersendiri, tetapi API boleh menyediakan derived representation untuk frontend.

```http
GET /api/v1/courses/{course_id}/progress
Authorization: Bearer <token>
```

Response:

```json
{
  "data": {
    "course_id": "uuid",
    "completed_lessons": 3,
    "total_lessons": 5,
    "percentage": 60,
    "recommended_lesson_id": "uuid"
  }
}
```

Endpoint ini **menghitung state**, bukan membaca `course_progress`.

---

# 12. Lesson

## 12.1 Get Lesson

```http
GET /api/v1/lessons/{lesson_id}
Authorization: Bearer <token>
```

Response:

```json
{
  "data": {
    "id": "uuid",
    "course_id": "uuid",
    "title": "Variables",
    "slug": "variables",
    "description": "Learn variables.",
    "order": 1,
    "status": "published"
  }
}
```

---

# 13. Lesson Exercises

```http
GET /api/v1/lessons/{lesson_id}/exercises
Authorization: Bearer <token>
```

Response:

```json
{
  "data": [
    {
      "id": "uuid",
      "title": "Create a Variable",
      "slug": "create-a-variable",
      "description": "Create a variable named x.",
      "order": 1,
      "xp": 100,
      "completed": true
    },
    {
      "id": "uuid",
      "title": "Update a Variable",
      "slug": "update-a-variable",
      "description": "Update the value of x.",
      "order": 2,
      "xp": 100,
      "completed": false
    }
  ]
}
```

---

# 14. Exercise

## 14.1 Get Exercise

```http
GET /api/v1/exercises/{exercise_id}
Authorization: Bearer <token>
```

Response:

```json
{
  "data": {
    "id": "uuid",
    "lesson_id": "uuid",
    "title": "Create a Variable",
    "slug": "create-a-variable",
    "description": "Create a variable named x.",
    "starter_code": "let x = 10;",
    "order": 1,
    "xp": 100,
    "language": "javascript",
    "completed": false
  }
}
```

`language` berasal dari konfigurasi Exercise dan tidak dapat dipilih atau
ditimpa oleh client ketika melakukan submission.

Database menyimpan `exercise.language` sebagai source of truth runtime untuk
exercise tersebut.

---

# 15. Exercise Test Cases

Public test cases dapat ditampilkan kepada client.

```http
GET /api/v1/exercises/{exercise_id}/test-cases
Authorization: Bearer <token>
```

Response:

```json
{
  "data": [
    {
      "id": "uuid",
      "input": "5",
      "expected_output": "25",
      "order": 1
    }
  ]
}
```

Hidden test cases **tidak pernah dikirim ke client**.

Server menentukan test case mana yang boleh ditampilkan.

---

# 16. Submit Exercise

Ini merupakan endpoint utama untuk learning flow.

```http
POST /api/v1/exercises/{exercise_id}/submissions
Authorization: Bearer <token>
Content-Type: application/json
```

Request:

```json
{
  "code": "let x = 10;\nconsole.log(x);"
}
```

---

# 17. Submission Response

Untuk MVP dengan synchronous execution:

```http
200 OK
```

Response:

```json
{
  "data": {
    "submission": {
      "id": "uuid",
      "exercise_id": "uuid",
      "status": "PASSED",
      "execution_time": 42
    },
    "exercise": {
      "completed": true
    },
    "xp": {
      "awarded": 100
    }
  }
}
```

Jika submission gagal:

```json
{
  "data": {
    "submission": {
      "id": "uuid",
      "exercise_id": "uuid",
      "status": "FAILED",
      "execution_time": 41
    },
    "exercise": {
      "completed": false
    },
    "xp": {
      "awarded": 0
    }
  }
}
```

---

# 18. Submission Result

Execution dapat memiliki hasil internal yang lebih detail.

Contoh:

```text
QUEUED
RUNNING
PASSED
FAILED
TIMEOUT
COMPILE_ERROR
RUNTIME_ERROR
```

API learning dapat menyederhanakannya menjadi:

```text
PASSED
FAILED
```

Jika frontend membutuhkan informasi error:

```json
{
  "data": {
    "submission": {
      "id": "uuid",
      "status": "FAILED",
      "failure": {
        "type": "TIMEOUT",
        "message": "Execution timed out."
      }
    }
  }
}
```

Detail ini tetap berada di bawah control Execution module.

---

# 19. Submission History

User dapat mengambil submission history untuk sebuah exercise.

```http
GET /api/v1/exercises/{exercise_id}/submissions
Authorization: Bearer <token>
```

Response:

```json
{
  "data": [
    {
      "id": "uuid",
      "status": "FAILED",
      "execution_time": 40,
      "created_at": "2026-09-09T10:00:00Z"
    },
    {
      "id": "uuid",
      "status": "PASSED",
      "execution_time": 42,
      "created_at": "2026-09-09T10:02:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 2
  }
}
```

User hanya dapat melihat submission miliknya sendiri.

---

# 20. User Progress

Progress user dapat disediakan sebagai derived API.

```http
GET /api/v1/me/progress
Authorization: Bearer <token>
```

Response:

```json
{
  "data": {
    "courses": [
      {
        "course_id": "uuid",
        "completed_lessons": 3,
        "total_lessons": 5,
        "percentage": 60,
        "recommended_lesson_id": "uuid"
      }
    ]
  }
}
```

Tidak ada:

```text
GET /progress
POST /progress
PUT /progress
```

karena progress bukan entity yang disimpan.

---

# 21. XP

## 21.1 Current XP

```http
GET /api/v1/me/xp
Authorization: Bearer <token>
```

Response:

```json
{
  "data": {
    "total": 400
  }
}
```

Total dihitung dari:

```text
SUM(gamification.xp_transactions.amount)
```

---

# 22. XP History

```http
GET /api/v1/me/xp/transactions
Authorization: Bearer <token>
```

Response:

```json
{
  "data": [
    {
      "id": "uuid",
      "amount": 100,
      "type": "EXERCISE_COMPLETION",
      "exercise_id": "uuid",
      "created_at": "2026-09-09T10:02:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 4
  }
}
```

User hanya dapat melihat transaction miliknya sendiri.

---

# 23. Recommended Lesson

Recommended lesson dapat diperoleh melalui course progress.

```http
GET /api/v1/courses/{course_id}/progress
```

Response:

```json
{
  "data": {
    "completed_lessons": 2,
    "total_lessons": 5,
    "percentage": 40,
    "recommended_lesson_id": "uuid"
  }
}
```

Tidak terdapat endpoint mutation:

```text
POST /recommended-lesson
```

Recommendation adalah derived state.

---

# 24. Authentication Rules

Endpoint dibagi menjadi:

### Public

```text
POST /auth/register
POST /auth/login

GET /courses
GET /courses/{id}
GET /courses/slug/{slug}
```

### Authenticated

```text
POST /auth/logout
GET /me

GET /courses/{id}/lessons
GET /courses/{id}/progress

GET /lessons/{id}
GET /lessons/{id}/exercises

GET /exercises/{id}
GET /exercises/{id}/test-cases
POST /exercises/{id}/submissions
GET /exercises/{id}/submissions

GET /me/progress
GET /me/xp
GET /me/xp/transactions
```

---

# 25. Authorization

User hanya dapat mengakses data personal miliknya.

Contoh:

```text
GET /exercises/{id}/submissions
```

Backend harus selalu menggunakan authenticated user:

```text
submission.user_id = current_user.id
```

Client tidak boleh menentukan:

```json
{
  "user_id": "another-user-id"
}
```

User identity selalu berasal dari authentication context.

---

# 26. Error Response

Semua error menggunakan format konsisten.

```json
{
  "error": {
    "code": "EXERCISE_NOT_FOUND",
    "message": "Exercise not found."
  }
}
```

Validation error:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Request validation failed.",
    "fields": {
      "code": [
        "Code is required."
      ]
    }
  }
}
```

---

# 27. HTTP Status Codes

| Status                      | Usage                                   |
| --------------------------- | --------------------------------------- |
| `200 OK`                    | Successful read/action                  |
| `201 Created`               | Resource successfully created           |
| `204 No Content`            | Successful action without response body |
| `400 Bad Request`           | Invalid request                         |
| `401 Unauthorized`          | Authentication required/invalid         |
| `403 Forbidden`             | Authenticated but not authorized        |
| `404 Not Found`             | Resource does not exist                 |
| `409 Conflict`              | Request conflicts with current state    |
| `422 Unprocessable Entity`  | Validation/domain input failure         |
| `429 Too Many Requests`     | Rate limit exceeded                     |
| `500 Internal Server Error` | Unexpected server error                 |

---

# 28. Error Codes

Initial error codes:

```text
AUTH_INVALID_CREDENTIALS
AUTH_UNAUTHORIZED

USER_NOT_FOUND

COURSE_NOT_FOUND
LESSON_NOT_FOUND
EXERCISE_NOT_FOUND

SUBMISSION_NOT_FOUND

VALIDATION_ERROR

EXECUTION_TIMEOUT
EXECUTION_FAILED
EXECUTION_RUNTIME_ERROR
EXECUTION_COMPILE_ERROR

RATE_LIMIT_EXCEEDED
INTERNAL_ERROR
```

Error codes merupakan stable contract untuk frontend.

`message` dapat berubah tanpa dianggap breaking change.

---

# 29. Pagination

Collection endpoint menggunakan pagination.

Query:

```text
?page=1&limit=20
```

Response:

```json
{
  "data": [],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 100
  }
}
```

Default:

```text
page = 1
limit = 20
```

Maximum:

```text
limit = 100
```

Pagination digunakan terutama untuk:

```text
GET /courses
GET /exercises/{id}/submissions
GET /me/xp/transactions
```

Course lesson/exercise collection yang kecil dapat menggunakan non-paginated response jika jumlahnya tetap terbatas.

---

# 30. Ordering

Lesson dan exercise dikembalikan berdasarkan:

```text
order ASC
```

Example:

```text
Lesson 1
Lesson 2
Lesson 3
```

Client tidak perlu melakukan sorting berdasarkan `order`.

---

# 31. Idempotency

Submission merupakan operation yang menghasilkan execution dan tidak dianggap idempotent.

Contoh:

```text
POST /exercises/{id}/submissions
```

Setiap request merupakan attempt baru.

Namun XP awarding **harus idempotent**.

Repeated successful submission:

```text
Submission 1 → PASSED → +100 XP
Submission 2 → PASSED → +0 XP
Submission 3 → PASSED → +0 XP
```

---

# 32. API Response Envelope

Single resource:

```json
{
  "data": {}
}
```

Collection:

```json
{
  "data": [],
  "meta": {}
}
```

Error:

```json
{
  "error": {}
}
```

API tidak mencampurkan:

```json
{
  "data": {},
  "error": {}
}
```

dalam normal response.

---

# 33. Content Visibility

Client hanya menerima:

```text
status = PUBLISHED
```

untuk public learning content.

Draft:

```text
DRAFT
```

hanya dapat diakses oleh future admin/content-management functionality.

Admin API merupakan bagian MVP sesuai PRD, tetapi detail endpoint-nya belum
didefinisikan dalam revisi contract ini. Jangan mengimplementasikan endpoint
admin berdasarkan asumsi; lengkapi contract terlebih dahulu.

---

# 34. Learning Flow

End-to-end learning flow:

```text
GET /courses
       ↓
GET /courses/{course_id}
       ↓
GET /courses/{course_id}/lessons
       ↓
GET /lessons/{lesson_id}/exercises
       ↓
GET /exercises/{exercise_id}
       ↓
GET /exercises/{exercise_id}/test-cases
       ↓
POST /exercises/{exercise_id}/submissions
       ↓
Execution
       ↓
PASSED / FAILED
       ↓
Exercise completion
       ↓
XP award
       ↓
Lesson completion
       ↓
Course progress
```

---

# 35. Submission Flow

```text
Client
  │
  │ POST submission
  ▼
API
  │
  ▼
Execution Module
  │
  ├── Validate
  ├── Execute
  ├── Run test cases
  └── Produce result
          │
          ▼
     PASSED / FAILED
          │
          ▼
Learning completion
          │
          ▼
Gamification
          │
          ▼
XP Transaction
```

---

# 36. API Does Not Expose Domain Internals

API tidak harus mengikuti entity database satu banding satu.

Contoh database:

```text
gamification.xp_transactions
```

API:

```text
GET /me/xp
```

Bukan:

```text
GET /xp_transactions
```

Begitu juga:

```text
learning.exercise_test_cases
```

dapat direpresentasikan sebagai:

```text
GET /exercises/{id}/test-cases
```

API merepresentasikan use case/resource yang diperlukan client.

---

# 37. MVP Endpoint Summary

```text
Authentication
────────────────────────────────────────
POST   /api/v1/auth/register
POST   /api/v1/auth/login
POST   /api/v1/auth/logout
GET    /api/v1/me


Courses
────────────────────────────────────────
GET    /api/v1/courses
GET    /api/v1/courses/{course_id}
GET    /api/v1/courses/slug/{slug}
GET    /api/v1/courses/{course_id}/lessons
GET    /api/v1/courses/{course_id}/progress


Lessons
────────────────────────────────────────
GET    /api/v1/lessons/{lesson_id}
GET    /api/v1/lessons/{lesson_id}/exercises


Exercises
────────────────────────────────────────
GET    /api/v1/exercises/{exercise_id}
GET    /api/v1/exercises/{exercise_id}/test-cases
POST   /api/v1/exercises/{exercise_id}/submissions
GET    /api/v1/exercises/{exercise_id}/submissions


User Progress & Gamification
────────────────────────────────────────
GET    /api/v1/me/progress
GET    /api/v1/me/xp
GET    /api/v1/me/xp/transactions
```

---

# 38. Contract Sections Not Yet Defined

Endpoint berikut belum didefinisikan dalam revisi contract ini. Item yang
termasuk MVP pada PRD tetap berada dalam scope dan harus memperoleh contract
sebelum diimplementasikan:

```text
POST /course-progress
POST /lesson-progress
POST /exercise-progress

POST /recommended-lessons

POST /xp

GET /leaderboard

GET /achievements

GET /streak

Practice problem endpoints
Playground run endpoint
Level endpoint/representation
Admin content-management endpoints
```

Leaderboard, achievements, dan streak tetap di luar MVP sesuai PRD. Practice,
Playground, Level, dan Admin adalah MVP dan bukan fitur opsional.

---

# 39. API Versioning

API menggunakan URL versioning:

```text
/api/v1
```

Breaking changes akan menggunakan versi baru:

```text
/api/v2
```

Non-breaking changes dapat dilakukan pada `v1`.

Contoh non-breaking:

```text
Menambahkan optional response field.
```

Contoh breaking:

```text
Mengubah struktur response.
Menghapus field.
Mengubah semantic field.
Mengubah required request field.
```

---

# 40. API Contract Principle

API mengikuti prinsip:

> **Expose capabilities, not database tables.**

Frontend berinteraksi dengan capability platform:

```text
Browse courses
View lesson
View exercise
Submit solution
View progress
View XP
```

bukan dengan struktur database internal.

Database/module architecture dapat berubah tanpa harus mengubah public API selama contract tetap kompatibel.
