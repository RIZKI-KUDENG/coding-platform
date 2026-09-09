# Execution Architecture

## 1. Overview

Execution system bertanggung jawab menjalankan source code yang dikirim user dan memvalidasi hasilnya terhadap exercise test cases.

Execution merupakan module terpisah di dalam Modular Monolith.

```text
┌───────────────────────────────────────────────┐
│              Modular Monolith                 │
│                                               │
│  ┌──────────────┐                             │
│  │ API          │                             │
│  └──────┬───────┘                             │
│         │                                     │
│  ┌──────▼───────────────────────────────────┐ │
│  │ Execution Module                         │ │
│  │                                          │ │
│  │ Submission                               │ │
│  │ Execution Orchestration                 │ │
│  │ Result Evaluation                       │ │
│  └──────┬───────────────────────────────────┘ │
│         │                                     │
└─────────┼─────────────────────────────────────┘
          │
          ▼
   ┌───────────────┐
   │ Isolated      │
   │ Code Runner   │
   └───────┬───────┘
           │
           ▼
      Test Cases
```

Tujuan utama:

1. Menjalankan untrusted user code.
2. Mencegah code user mengganggu API/backend host.
3. Membatasi resource.
4. Menjalankan test cases.
5. Menghasilkan execution result yang konsisten.
6. Menyimpan submission history.
7. Menjadi dasar untuk scaling execution di masa depan.

---

# 2. Goals

Execution system MVP harus:

* menjalankan code secara terisolasi
* memiliki timeout
* memiliki CPU limit
* memiliki memory limit
* tidak memiliki akses network
* menggunakan filesystem sementara
* membersihkan environment setelah execution
* menjalankan public dan hidden test cases
* menentukan PASS/FAIL
* menangani runtime/compile error
* menyimpan submission
* mencegah XP diberikan lebih dari sekali

---

# 3. Non-Goals

MVP tidak membutuhkan:

```text
Kubernetes
Kafka
RabbitMQ
Distributed execution cluster
Multi-region runner
Autoscaling
GPU execution
Persistent execution containers
```

Architecture harus dapat berkembang ke arah tersebut tanpa mengharuskannya dari hari pertama.

---

# 4. Core Architecture

MVP menggunakan:

```text
Rust API
   │
   ▼
Execution Application Service
   │
   ▼
Runner Adapter
   │
   ▼
Container Runtime
   │
   ▼
Ephemeral Container
```

Runner tidak dijalankan langsung sebagai:

```text
std::process::Command
```

dari API process tanpa isolation.

Host process harus mengontrol container runtime.

---

# 5. Execution Boundary

Ada dua security boundary:

```text
                TRUSTED
┌──────────────────────────────────────┐
│ API / Application                   │
│                                      │
│ Learning                             │
│ Execution                            │
│ Gamification                         │
└──────────────────┬───────────────────┘
                   │
                   │ controlled interface
                   ▼
                UNTRUSTED
┌──────────────────────────────────────┐
│ Code Runner Container                │
│                                      │
│ User source code                     │
│ Compiler / Runtime                   │
│ Test execution                       │
└──────────────────────────────────────┘
```

User code dianggap **untrusted input**.

Tidak boleh ada asumsi bahwa source code user aman.

---

# 6. Execution Flow

Flow utama:

```text
Client
  │
  │ POST /exercises/{id}/submissions
  ▼
API
  │
  ▼
Execution Application Service
  │
  ├── Authenticate user
  ├── Load exercise
  ├── Resolve language
  ├── Validate request
  └── Create submission
          │
          ▼
      Runner Adapter
          │
          ▼
     Create container
          │
          ▼
     Write source code
          │
          ▼
     Execute
          │
          ├── timeout
          ├── CPU limit
          ├── memory limit
          └── network disabled
          │
          ▼
      Run test cases
          │
          ▼
      Collect result
          │
          ▼
   Destroy container
          │
          ▼
   Update submission
          │
          ├── PASSED
          └── FAILED
          │
          ▼
  Exercise completion
          │
          ▼
    XP award if first pass
```

---

# 7. Synchronous Execution

Untuk MVP, execution menggunakan **synchronous request-response**.

```text
POST submission
       │
       ▼
execute code
       │
       ▼
return result
```

Client menerima result dalam request yang sama.

Contoh:

```text
POST /api/v1/exercises/{id}/submissions

      ↓

200 OK

{
  "submission": {
    "status": "PASSED"
  }
}
```

### Alasan

Jumlah user awal belum membutuhkan queue/distributed execution.

Synchronous execution:

* lebih sederhana
* lebih mudah debug
* lebih sedikit infrastructure
* lebih mudah local development
* tidak membutuhkan Redis/queue

---

# 8. Execution Timeout

Setiap execution memiliki hard timeout.

Contoh initial configuration:

```text
Execution timeout: 5 seconds
```

Timeout harus diterapkan dari luar proses user code.

Tidak boleh mengandalkan user code untuk menghentikan dirinya sendiri.

Example:

```text
while (true) {}
```

harus dihentikan oleh execution controller.

Flow:

```text
Start
  │
  ▼
Execute
  │
  ├── finished → continue
  │
  └── timeout → terminate
```

Timeout menghasilkan:

```text
TIMEOUT
```

yang kemudian dipetakan sebagai failed submission pada learning layer.

---

# 9. Resource Limits

Runner harus memiliki resource limits.

Initial MVP configuration:

```text
CPU:
1 CPU

Memory:
128 MB

Execution timeout:
5 seconds

Process limit:
limited

Filesystem:
temporary

Network:
disabled
```

Nilai tersebut merupakan initial configuration, bukan immutable domain rules.

Configuration harus dapat diubah tanpa mengubah domain model.

Contoh:

```text
EXECUTION_TIMEOUT_SECONDS=5
EXECUTION_MEMORY_MB=128
EXECUTION_CPU_LIMIT=1
```

---

# 10. Network Isolation

User code tidak membutuhkan network untuk exercise dasar.

Karena itu:

```text
Network = disabled
```

Runner tidak boleh:

```text
HTTP request
TCP connection
DNS lookup
Internet access
```

Contoh code:

```javascript
fetch("https://example.com")
```

harus gagal.

Tujuannya:

* mencegah SSRF
* mencegah port scanning
* mencegah abuse network
* mencegah data exfiltration
* mengurangi attack surface

---

# 11. Filesystem Isolation

Setiap execution menggunakan filesystem sementara.

```text
Host
 │
 ├── runner container
 │      └── /workspace
 │
 └── application data
```

User code hanya dapat mengakses filesystem container.

Tidak boleh ada mount seperti:

```text
/
./
/var/run/docker.sock
application directory
database files
```

Filesystem dibersihkan setelah execution.

---

# 12. Container Lifecycle

Container bersifat ephemeral.

```text
CREATE
  ↓
START
  ↓
EXECUTE
  ↓
COLLECT RESULT
  ↓
STOP
  ↓
REMOVE
```

Container tidak disimpan setelah execution selesai.

Jika execution timeout:

```text
TIMEOUT
  ↓
FORCE TERMINATE
  ↓
REMOVE CONTAINER
```

Cleanup harus terjadi baik pada:

```text
success
failure
timeout
runtime error
unexpected runner error
```

---

# 13. Container Image

Setiap language memiliki runtime image sendiri.

Contoh:

```text
runner-javascript
runner-rust
```

Future:

```text
runner-python
runner-go
runner-java
```

Struktur:

```text
Execution
   │
   ├── language = javascript
   │       ↓
   │   runner-javascript
   │
   └── language = rust
           ↓
       runner-rust
```

Image harus immutable/versioned.

Contoh:

```text
runner-javascript:v1
runner-rust:v1
```

Jangan menggunakan:

```text
latest
```

sebagai production execution dependency.

---

# 14. Language Resolution

Language berasal dari exercise.

```text
Exercise
  │
  └── language
        │
        ▼
      Runner image
```

Submission tidak menentukan language sendiri.

Request:

```json
{
  "code": "..."
}
```

bukan:

```json
{
  "language": "javascript",
  "code": "..."
}
```

Backend menentukan execution environment berdasarkan exercise.

Ini mencegah user menjalankan JavaScript exercise menggunakan runtime lain.

---

# 15. Test Case Execution

Exercise memiliki:

```text
Public Test Cases
Hidden Test Cases
```

Runner menjalankan seluruh test cases yang diperlukan.

```text
Exercise
  │
  ├── Test Case 1
  ├── Test Case 2
  ├── Test Case 3
  └── Test Case 4
```

Result:

```text
Test 1 ✓
Test 2 ✓
Test 3 ✓
Test 4 ✓

→ PASSED
```

Jika salah satu gagal:

```text
Test 1 ✓
Test 2 ✗
Test 3 ✓
Test 4 ✓

→ FAILED
```

---

# 16. Pass Rule

Exercise dianggap passed hanya jika **seluruh required test cases berhasil**.

```text
ALL TEST CASES PASSED
        │
        ▼
      PASSED
```

Jika:

```text
ANY TEST CASE FAILED
```

maka:

```text
FAILED
```

Tidak ada partial pass.

---

# 17. Test Case Isolation

Setiap test case sebaiknya tidak bergantung pada state test case sebelumnya.

Conceptual model:

```text
Test Case 1
   ↓
isolated execution

Test Case 2
   ↓
isolated execution

Test Case 3
   ↓
isolated execution
```

Dengan demikian:

```text
global state
filesystem
process state
```

tidak menyebabkan hasil test berikutnya bergantung pada test sebelumnya.

Implementation dapat mengoptimalkan lifecycle kemudian jika diperlukan, tetapi correctness harus menjadi prioritas.

---

# 18. Execution Result

Execution layer dapat menghasilkan:

```text
PASSED
FAILED
TIMEOUT
COMPILE_ERROR
RUNTIME_ERROR
```

Result internal dapat memiliki detail:

```json
{
  "status": "RUNTIME_ERROR",
  "execution_time": 42,
  "error": {
    "message": "ReferenceError: x is not defined"
  }
}
```

Learning layer memetakan result menjadi:

```text
PASSED
FAILED
```

untuk completion logic.

---

# 19. Submission Lifecycle

Secara konseptual:

```text
RECEIVED
   ↓
RUNNING
   ↓
COMPLETED
```

Completion result:

```text
PASSED
FAILED
TIMEOUT
COMPILE_ERROR
RUNTIME_ERROR
```

Namun persistence MVP dapat menyimpan hasil final yang diperlukan oleh API.

Jika asynchronous execution diperkenalkan nanti, lifecycle:

```text
QUEUED
RUNNING
COMPLETED
FAILED
```

dapat ditambahkan tanpa mengubah konsep submission.

---

# 20. Submission Persistence

Submission disimpan sebelum atau selama execution sehingga setiap attempt memiliki identity.

```text
Create Submission
       │
       ▼
submission.id
       │
       ▼
Execute
       │
       ▼
Update result
```

Hal ini memastikan setiap attempt memiliki identifier yang stabil.

---

# 21. Failure Handling

Failure dibagi menjadi dua kategori.

### User Code Failure

```text
COMPILE_ERROR
RUNTIME_ERROR
TIMEOUT
TEST_FAILURE
```

Ini merupakan hasil normal execution.

Submission:

```text
FAILED
```

### Infrastructure Failure

Contoh:

```text
Container failed to start
Runner unavailable
Container runtime error
Unexpected internal error
```

Ini bukan kesalahan user.

Execution layer harus menghasilkan internal error dan API dapat mengembalikan:

```text
500 Internal Server Error
```

atau service-unavailable response sesuai kondisi.

Jangan mengubah infrastructure failure menjadi:

```text
FAILED
```

karena itu akan menyesatkan user dan analytics.

---

# 22. Retry Policy

Untuk MVP:

```text
User code failure
→ NO RETRY

Infrastructure failure
→ limited retry
```

User tidak boleh menyebabkan execution otomatis diulang hanya karena:

```text
runtime error
timeout
test failure
```

Infrastructure retry dapat ditambahkan dengan batas kecil.

Contoh:

```text
Container startup failure
→ retry once
→ if still fails → infrastructure error
```

---

# 23. Concurrency

MVP tidak memerlukan distributed scheduler.

Runner dapat membatasi jumlah concurrent executions.

Contoh:

```text
Maximum concurrent executions = N
```

Nilai `N` merupakan configuration berdasarkan resource host.

Jika limit tercapai:

```text
new execution
     ↓
wait / reject
```

Untuk synchronous MVP, lebih aman menggunakan bounded concurrency daripada membiarkan user membuka execution sebanyak mungkin.

---

# 24. Rate Limiting

Submission endpoint harus memiliki rate limit.

Contoh initial policy:

```text
POST /exercises/{id}/submissions
```

dibatasi berdasarkan:

```text
user_id
IP
```

Policy exact value dapat ditentukan pada infrastructure/security configuration.

Tujuannya mencegah:

```text
submission spam
CPU exhaustion
memory exhaustion
runner abuse
```

Rate limiting harus berada sebelum execution dimulai.

---

# 25. Execution Security Rules

Runner harus memastikan:

```text
No network
No privileged container
No host filesystem mount
No Docker socket
No host PID namespace
No host network
Limited CPU
Limited memory
Limited processes
Hard timeout
Ephemeral filesystem
```

Container tidak boleh dijalankan dengan:

```text
--privileged
```

dan tidak boleh memiliki akses terhadap container runtime socket.

---

# 26. Secrets Isolation

Environment runner tidak boleh menerima application secrets.

Jangan inject:

```text
DATABASE_URL
JWT_SECRET
API_KEYS
SESSION_SECRET
```

ke execution container.

User code harus berjalan tanpa akses ke application credentials.

---

# 27. Logging

Execution harus menghasilkan operational logs.

Minimal:

```text
submission_id
user_id
exercise_id
language
execution_duration
execution_result
runner_error
```

Jangan menyimpan source code di application log.

Code merupakan user data dan dapat mengandung informasi sensitif.

---

# 28. Observability

Initial metrics:

```text
execution_total
execution_success_total
execution_failure_total
execution_timeout_total
execution_duration
runner_start_failure_total
```

Useful latency metrics:

```text
p50
p95
p99
```

Metrics digunakan untuk mengetahui:

* runner terlalu lambat
* timeout terlalu sering
* resource limit terlalu rendah
* infrastructure failure
* abuse pattern

---

# 29. Data Flow

```text
                 ┌──────────────┐
                 │   Client     │
                 └──────┬───────┘
                        │
                        ▼
                 ┌──────────────┐
                 │ API          │
                 └──────┬───────┘
                        │
                        ▼
              ┌────────────────────┐
              │ Execution Service  │
              └─────────┬──────────┘
                        │
                 create submission
                        │
                        ▼
              ┌────────────────────┐
              │ Runner Adapter     │
              └─────────┬──────────┘
                        │
                        ▼
              ┌────────────────────┐
              │ Container Runtime  │
              └─────────┬──────────┘
                        │
                        ▼
              ┌────────────────────┐
              │ Ephemeral Runner   │
              │                    │
              │ User Code          │
              │ Test Cases         │
              └─────────┬──────────┘
                        │
                        ▼
                 Execution Result
                        │
                        ▼
              ┌────────────────────┐
              │ Submission         │
              └─────────┬──────────┘
                        │
                ┌───────┴────────┐
                ▼                ▼
          Completion         XP Award
```

---

# 30. Application Layer Boundary

Execution module dapat memiliki struktur konseptual:

```text
execution/
├── domain/
│   ├── submission
│   ├── execution_result
│   └── runner
│
├── application/
│   ├── submit_solution
│   └── execute_submission
│
├── infrastructure/
│   ├── container_runner
│   └── persistence
│
└── presentation/
    └── http
```

Domain tidak mengetahui Docker/container implementation.

Contoh:

```text
Domain
  │
  ▼
RunnerPort
  │
  ▼
DockerRunner
```

Dengan demikian runner implementation dapat diganti kemudian.

---

# 31. Runner Port

Application layer bergantung pada abstraction:

```text
Runner
```

Conceptually:

```text
run(
    language,
    source_code,
    test_cases,
    limits
) -> ExecutionResult
```

Infrastructure menyediakan implementation:

```text
ContainerRunner
```

Future implementation:

```text
RemoteRunner
DistributedRunner
KubernetesRunner
```

tidak memerlukan perubahan pada domain logic.

---

# 32. Local Development

Local development dapat menggunakan container runtime yang sama dengan production.

```text
Rust API
   │
   ▼
Container Runtime
   │
   ▼
Runner Container
```

Developer dapat menjalankan:

```text
docker/podman
```

dengan runner images lokal.

Tujuan utamanya adalah membuat behavior development mendekati production.

---

# 33. Production Deployment — MVP

Untuk awal, API dan runner dapat berada pada **host/VM yang sama**, tetapi tetap dipisahkan melalui container boundary.

```text
VM
│
├── Rust API Container
│
├── PostgreSQL
│
└── Execution Runner
      └── ephemeral containers
```

Namun runner container harus tetap memiliki resource/security isolation.

Tidak diperlukan VM terpisah pada tahap awal jika threat model dan resource limits memadai.

---

# 34. Future Scaling

Ketika jumlah execution meningkat:

```text
MVP
────────────────
API
 │
 └── Local Runner


Future
────────────────
API
 │
 ▼
Execution Queue
 │
 ▼
Runner Pool
 ├── Runner 1
 ├── Runner 2
 ├── Runner 3
 └── Runner N
```

Kemudian dapat berkembang menjadi:

```text
API
 │
 ▼
Queue
 │
 ▼
Execution Workers
 │
 ├── JavaScript Workers
 ├── Rust Workers
 ├── Python Workers
 └── ...
```

Architecture MVP tidak boleh mengharuskan perubahan domain ketika ini dilakukan.

---

# 35. Synchronous → Asynchronous Migration

MVP:

```text
POST submission
     │
     ▼
execute
     │
     ▼
response
```

Future:

```text
POST submission
     │
     ▼
create submission
     │
     ▼
QUEUED
     │
     ▼
202 Accepted
```

Client kemudian melakukan:

```text
GET /submissions/{id}
```

atau menggunakan realtime mechanism.

Karena submission sudah memiliki ID dan lifecycle, migration dapat dilakukan tanpa mengganti konsep domain utama.

---

# 36. Important Security Principle

Execution environment harus diperlakukan sebagai hostile environment.

Jangan mengandalkan:

```text
"User hanya belajar coding"
```

sebagai security boundary.

Anggap user code dapat mencoba:

```text
Infinite loop
Memory exhaustion
Fork bomb
Filesystem traversal
Network access
Process inspection
Environment variable inspection
Container escape
Resource abuse
```

Mitigasi harus berada di infrastructure boundary, bukan di validation source code saja.

---

# 37. MVP Configuration

Initial configuration:

```text
Execution timeout:
5s

Memory:
128MB

CPU:
1 CPU

Network:
disabled

Filesystem:
ephemeral

Process limit:
restricted

Container:
ephemeral

Execution:
synchronous

Queue:
none

Runner:
same host

Retry:
infrastructure only
```

Nilai ini harus dapat dikonfigurasi.

---

# 38. Decisions

Execution Architecture MVP menggunakan:

```text
Architecture
→ Modular Monolith

Execution
→ isolated containers

Communication
→ in-process application call

Execution mode
→ synchronous

Queue
→ none

Network
→ disabled

Filesystem
→ ephemeral

Timeout
→ hard timeout

Resource limits
→ CPU + memory + process limits

Runner lifecycle
→ ephemeral

Language
→ configured by Exercise

Test validation
→ all required tests must pass

XP
→ awarded once after first pass
```

---

# 39. Explicit Non-Decisions

Hal berikut belum dikunci karena belum dibutuhkan MVP:

```text
Exact container runtime
Exact container image build pipeline
Exact CPU quota configuration
Exact memory configuration
Distributed runner protocol
Queue technology
Worker orchestration
Kubernetes
Multi-host execution
```

Implementation detail tersebut dapat dipilih pada Infrastructure Design berdasarkan deployment environment.

---

# 40. Final Architecture

MVP execution architecture:

```text
                       ┌───────────────┐
                       │    Client     │
                       └───────┬───────┘
                               │
                               ▼
                    ┌────────────────────┐
                    │     Rust API       │
                    └─────────┬──────────┘
                              │
                              ▼
                    ┌────────────────────┐
                    │ Execution Module   │
                    │                    │
                    │ Submission         │
                    │ Orchestration      │
                    │ Result Mapping     │
                    └─────────┬──────────┘
                              │
                         Runner Port
                              │
                              ▼
                    ┌────────────────────┐
                    │ Container Runner   │
                    └─────────┬──────────┘
                              │
                     ephemeral container
                              │
                              ▼
                 ┌──────────────────────────┐
                 │      User Code           │
                 │                          │
                 │  CPU limited             │
                 │  Memory limited          │
                 │  Timeout enforced        │
                 │  Network disabled        │
                 │  Filesystem isolated     │
                 └────────────┬─────────────┘
                              │
                              ▼
                       Test Results
                              │
                              ▼
                    Submission Result
                              │
                  ┌───────────┴───────────┐
                  ▼                       ▼
             Completion               XP Award
```

Architecture ini cukup sederhana untuk MVP, tetapi memiliki boundary yang memungkinkan execution system dipisahkan dan diskalakan ketika traffic meningkat.
