# Infrastructure Design

## 1. Overview

Platform menggunakan infrastructure sederhana untuk MVP:

```text
                         Internet
                            │
                            ▼
                    ┌──────────────┐
                    │ Reverse Proxy│
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ Rust API     │
                    │ Modular      │
                    │ Monolith     │
                    └──────┬───────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
             ▼                           ▼
      ┌──────────────┐          ┌────────────────┐
      │ PostgreSQL   │          │ Podman         │
      │              │          │ Runner         │
      └──────────────┘          └───────┬────────┘
                                        │
                                        ▼
                              Ephemeral Containers
```

---

# 2. Infrastructure Goals

Infrastructure harus:

* mudah dijalankan secara lokal
* murah untuk MVP
* sederhana untuk deployment
* memiliki security boundary untuk code execution
* dapat diobservasi
* dapat di-backup
* dapat ditingkatkan tanpa rewrite architecture

---

# 3. Infrastructure Non-Goals

MVP tidak menggunakan:

```text
Kubernetes
Redis
Kafka
RabbitMQ
Service Mesh
Distributed tracing infrastructure
Multi-region deployment
Autoscaling cluster
Dedicated execution cluster
```

Komponen tersebut hanya ditambahkan ketika ada kebutuhan nyata.

---

# 4. Environment

Minimal terdapat dua environment:

```text
development
production
```

Future:

```text
staging
```

Development harus sedekat mungkin dengan production pada bagian execution.

---

# 5. Local Development

Local architecture:

```text
Developer Machine
│
├── Astro Dev Server
│
├── Rust API
│
├── PostgreSQL
│
└── Podman
    └── Ephemeral Runner Containers
```

Podman digunakan sebagai container runtime lokal.

Docker Desktop tidak menjadi dependency development.

---

# 6. Local PostgreSQL

PostgreSQL dapat dijalankan melalui Podman.

```text
Podman
│
├── PostgreSQL container
│
└── Runner containers
```

Database menggunakan persistent volume.

```text
postgres container
       │
       ▼
postgres_data volume
```

Database tidak boleh menggunakan ephemeral filesystem seperti runner.

---

# 7. Local Runner

Runner menggunakan container image berdasarkan language.

Contoh:

```text
runner-javascript:v1
runner-rust:v1
```

Execution:

```text
Rust API
   │
   ▼
Podman
   │
   ▼
runner-javascript:v1
   │
   ▼
execute
   │
   ▼
destroy
```

Container runner tidak menggunakan persistent volume.

---

# 8. Production MVP

Production dapat menggunakan satu VM yang menjalankan:

```text
VM
│
├── Reverse Proxy
│
├── Rust API
│
├── PostgreSQL
│
└── Container Runtime
      │
      └── Ephemeral Runner Containers
```

Untuk MVP, tidak diperlukan cluster.

---

# 9. Recommended Production Separation

Walaupun dapat berada pada satu VM, runner harus tetap memiliki boundary:

```text
┌─────────────────────────────────────────┐
│                  VM                     │
│                                         │
│  ┌─────────────┐     ┌──────────────┐  │
│  │ Rust API    │     │ PostgreSQL   │  │
│  └──────┬──────┘     └──────────────┘  │
│         │                               │
│         ▼                               │
│  ┌──────────────────────────────┐       │
│  │ Container Runtime            │       │
│  │                              │       │
│  │  ┌────────────────────────┐  │       │
│  │  │ Untrusted Runner       │  │       │
│  │  └────────────────────────┘  │       │
│  └──────────────────────────────┘       │
│                                         │
└─────────────────────────────────────────┘
```

API tidak menjalankan user code sebagai host process.

---

# 10. Container Runtime

Local:

```text
Podman
```

Production:

```text
Container Runtime
```

Production runtime belum dikunci pada tahap ini.

Infrastructure abstraction tetap:

```text
ContainerRunner
```

sehingga application code tidak bergantung pada runtime tertentu.

---

# 11. Runner Security

Runner container harus:

```text
Network disabled
CPU limited
Memory limited
Process limited
Filesystem isolated
Non-privileged
Ephemeral
```

Tidak boleh:

```text
--privileged
host network
host PID namespace
host filesystem mount
Docker/Podman socket mount
application secret injection
```

---

# 12. Runner Network

Runner tidak membutuhkan network.

```text
Runner
  │
  X── Internet
  X── Internal services
  X── PostgreSQL
```

Runner hanya membutuhkan filesystem dan runtime/compiler yang sudah tersedia di image.

---

# 13. Application Network

Normal application:

```text
Internet
   │
   ▼
Reverse Proxy
   │
   ▼
Rust API
   │
   ├── PostgreSQL
   │
   └── Container Runtime
```

Database tidak exposed langsung ke internet.

---

# 14. Database

Database:

```text
PostgreSQL
```

PostgreSQL digunakan untuk:

* users
* courses
* lessons
* exercises
* test cases
* submissions
* XP transactions
* progression data

Database menggunakan schema per bounded context.

```text
identity
learning
execution
gamification
```

---

# 15. Database Access

API menggunakan connection pool.

Conceptual:

```text
Rust API
   │
   ▼
Connection Pool
   │
   ▼
PostgreSQL
```

Pool size harus dikonfigurasi berdasarkan resource VM dan workload.

Jangan langsung menggunakan connection pool besar.

---

# 16. Database Exposure

PostgreSQL:

```text
Public Internet
      X
      │
      ▼
Private application network
      │
      ▼
PostgreSQL
```

Hanya API/database administration path yang diperlukan yang dapat mengakses PostgreSQL.

---

# 17. Database Backup

Production harus memiliki backup PostgreSQL.

Minimal:

```text
Scheduled database backup
```

Backup harus berada di storage yang berbeda dari database primary.

Contoh conceptual:

```text
PostgreSQL
    │
    ▼
Backup
    │
    ▼
External Storage
```

Backup tidak boleh hanya berupa copy file di disk VM yang sama.

---

# 18. Database Migration

Migration menggunakan SQL migration files dari repository.

```text
migrations/
```

Deployment flow:

```text
Build
  │
  ▼
Run migrations
  │
  ▼
Start application
```

Migration harus backward-compatible jika deployment nantinya menggunakan rolling deployment.

Untuk single-instance MVP, migration dapat dilakukan sebelum application restart.

---

# 19. Secrets

Secrets tidak boleh berada di Git repository.

Contoh:

```text
DATABASE_URL
AUTH_SECRET
```

Production secrets diberikan melalui environment/secret management mechanism.

`.env.example` hanya berisi placeholder.

---

# 20. Configuration

Configuration dipisahkan dari code.

Contoh:

```text
APP_HOST
APP_PORT

DATABASE_URL
DATABASE_MAX_CONNECTIONS

AUTH_SECRET

EXECUTION_TIMEOUT_SECONDS
EXECUTION_MEMORY_MB
EXECUTION_CPU_LIMIT
EXECUTION_MAX_PROCESSES

RUNNER_RUNTIME
RUNNER_IMAGE_PREFIX
```

Nilai berbeda antara development dan production.

---

# 21. Reverse Proxy

Production menggunakan reverse proxy di depan API.

```text
Internet
   │
   ▼
Reverse Proxy
   │
   ▼
Rust API
```

Responsibilities:

* TLS termination
* HTTP routing
* request size limit
* basic connection protection
* forwarding headers

Rust API tidak perlu menangani TLS secara langsung pada MVP.

---

# 22. TLS

Production menggunakan HTTPS.

```text
Client
  │
 HTTPS
  ▼
Reverse Proxy
  │
 HTTP/private connection
  ▼
Rust API
```

Database dan runner tidak exposed melalui public HTTPS.

---

# 23. API Exposure

Public:

```text
443 → Reverse Proxy
```

Internal:

```text
Rust API → PostgreSQL
Rust API → Container Runtime
```

PostgreSQL tidak membuka port ke public internet.

---

# 24. Resource Allocation

MVP resource allocation harus konservatif.

Conceptual:

```text
VM
│
├── OS
├── Reverse Proxy
├── Rust API
├── PostgreSQL
└── Runner capacity
```

Runner execution memiliki explicit resource limit agar user code tidak menghabiskan seluruh VM.

Contoh initial runner:

```text
CPU:
1 CPU

Memory:
128 MB

Timeout:
5 seconds
```

Nilai dapat disesuaikan setelah benchmark.

---

# 25. Runner Concurrency

Host memiliki global execution concurrency limit.

Contoh:

```text
MAX_CONCURRENT_EXECUTIONS=N
```

Flow:

```text
Submission
    │
    ▼
Concurrency Check
    │
    ├── capacity available
    │       ↓
    │    execute
    │
    └── capacity full
            ↓
       wait/reject
```

Tujuan utamanya mencegah CPU exhaustion.

---

# 26. Submission Rate Limit

Endpoint submission memiliki rate limit.

```text
User
  │
  ▼
Rate Limit
  │
  ▼
Execution
```

Rate limit diterapkan sebelum container dibuat.

Hal ini mencegah user melakukan:

```text
submission spam
container spam
CPU exhaustion
```

---

# 27. Container Image Management

Runner images dibangun dari repository.

```text
runner/images/
├── javascript/
│   └── Containerfile
│
└── rust/
    └── Containerfile
```

Image diberi version.

```text
runner-javascript:v1
runner-rust:v1
```

Application configuration menentukan image yang digunakan.

---

# 28. Runner Image Principle

Runner image harus memiliki semua dependency yang dibutuhkan untuk execution.

Contoh JavaScript:

```text
Runtime
Test harness
Required system tools
```

Contoh Rust:

```text
Rust compiler
Standard libraries
Test harness
Required system tools
```

User code tidak boleh meng-install package dari internet saat execution.

---

# 29. Package Installation

Untuk MVP:

```text
No network package installation during execution
```

Jika suatu language membutuhkan dependencies, dependencies tersebut harus disediakan melalui controlled runner image atau dependency mechanism yang dirancang khusus.

Ini mencegah:

```text
npm install
cargo install
pip install
```

mengakses internet dari runner.

---

# 30. Logging

Application logs:

```text
timestamp
level
module
request_id
user_id
submission_id
error
```

Runner logs:

```text
submission_id
execution_id
language
duration
exit status
result
```

Source code tidak dimasukkan ke log.

---

# 31. Log Retention

MVP tidak membutuhkan centralized logging platform.

Application dapat menulis structured logs ke stdout/stderr.

Infrastructure kemudian menangani collection/retention.

Runner output yang diperlukan untuk user disimpan sebagai execution result, bukan sebagai permanent raw logs.

---

# 32. Metrics

Minimal metrics:

```text
http_requests_total
http_request_duration

db_connections
db_query_duration

execution_total
execution_duration
execution_timeout_total
execution_failure_total
runner_start_failure_total

submission_total
submission_passed_total
submission_failed_total
```

Metric execution merupakan prioritas karena runner adalah resource-sensitive component.

---

# 33. Health Checks

API menyediakan:

```text
/health
```

dan readiness check dapat digunakan untuk memastikan dependency penting tersedia.

Conceptual:

```text
GET /health

→ application process alive
```

Readiness dapat memeriksa:

```text
database connectivity
required infrastructure
```

Runner tidak perlu dipanggil hanya untuk health check.

---

# 34. Graceful Shutdown

API harus menangani shutdown dengan graceful behavior.

```text
Shutdown signal
      │
      ▼
Stop accepting requests
      │
      ▼
Finish active requests
      │
      ▼
Close DB pool
      │
      ▼
Exit
```

Execution yang sedang berjalan harus memiliki cleanup mechanism sehingga container tidak tertinggal.

---

# 35. Runner Cleanup

Cleanup harus terjadi jika:

```text
execution success
execution failed
timeout
API error
runner error
application shutdown
```

Target:

```text
No orphan containers
```

Monitoring dapat mendeteksi orphan containers jika cleanup gagal.

---

# 36. Deployment

MVP deployment:

```text
Git Repository
      │
      ▼
Build
      │
      ├── Rust API image
      └── Runner images
      │
      ▼
Deploy VM
      │
      ▼
Run migrations
      │
      ▼
Start API
```

CI/CD dapat ditambahkan sesuai workflow repository.

---

# 37. Deployment Order

Recommended:

```text
1. Build artifacts
2. Build runner images
3. Build API image
4. Pull/install images on production
5. Run database migration
6. Start/restart API
7. Health check
8. Verify runner
```

Runner image harus tersedia sebelum submission dapat diproses.

---

# 38. Application Container

API sendiri sebaiknya berjalan sebagai container.

```text
Rust API
   │
   ▼
api:v1
```

Sehingga production environment lebih reproducible.

Final production process:

```text
Reverse Proxy
      │
      ▼
API Container
      │
      ├── PostgreSQL
      │
      └── Container Runtime
             │
             ▼
       Runner Containers
```

---

# 39. Important Container Runtime Boundary

API membutuhkan kemampuan untuk meminta container runtime membuat runner.

Namun:

**API container tidak boleh mendapatkan unrestricted host access.**

Akses runtime harus dirancang dengan hati-hati karena runtime socket pada dasarnya merupakan privileged capability.

Untuk MVP, keputusan deployment ini perlu diverifikasi terhadap security model runtime yang dipilih.

Jika API container tidak dapat diberikan access secara aman, runner orchestration dapat dijalankan melalui dedicated host-side runner process.

---

# 40. Runner Orchestration Options

Ada dua opsi:

### Option A — API langsung mengontrol runtime

```text
API Container
     │
     ▼
Runtime API
     │
     ▼
Runner Container
```

Lebih sederhana, tetapi memberikan API process akses sensitif terhadap container runtime.

### Option B — Host Runner Agent

```text
API Container
     │
     ▼
Runner Agent
     │
     ▼
Container Runtime
     │
     ▼
Runner Container
```

Security boundary lebih baik, tetapi menambah component.

### MVP Decision

Gunakan **Option A hanya jika runtime access dapat dibatasi dengan aman**.

Jika tidak, gunakan **Option B**.

Jangan memberikan unrestricted runtime socket hanya demi mengurangi satu component.

---

# 41. Local Runtime

Local development:

```text
Rust API
   │
   ▼
Podman
   │
   ▼
Runner Container
```

Podman dipilih untuk local development karena tidak membutuhkan Docker Desktop sebagai dependency.

---

# 42. Production Runtime Decision

Production runtime belum dikunci dalam document ini.

Criteria:

```text
Security
Resource usage
Rootless capability
Container lifecycle control
Operational simplicity
Compatibility with VM
```

Pilihan runtime harus diputuskan berdasarkan deployment target aktual.

---

# 43. Scaling Path

Initial:

```text
1 VM
1 API
1 PostgreSQL
Runner containers
```

Scale-up:

```text
1 VM
API
PostgreSQL
Runner capacity
```

Scale-out:

```text
                ┌── API 1
Load Balancer ──┼── API 2
                └── API N
                     │
                     ▼
                Execution Queue
                     │
                     ▼
                Runner Workers
```

Database scaling dan execution scaling dilakukan secara independen.

---

# 44. When to Introduce Queue

Queue diperkenalkan ketika salah satu kondisi terjadi:

```text
Execution duration becomes problematic
High concurrent submissions
API request latency becomes unacceptable
Runner capacity requires independent scaling
Execution retries need durable scheduling
```

Bukan karena architecture diagram terlihat lebih modern.

---

# 45. When to Separate Runner Host

Runner host dipisahkan jika:

```text
User execution consumes significant resources
Security isolation requires stronger boundary
Runner traffic affects API/database
Execution workload becomes unpredictable
```

Future:

```text
API VM
   │
   ▼
Execution Queue
   │
   ▼
Runner VM(s)
```

---

# 46. Disaster Recovery

MVP priority:

```text
PostgreSQL backup
Application image reproducibility
Runner image reproducibility
Configuration backup
```

Runner containers sendiri tidak perlu di-backup karena ephemeral.

---

# 47. Data Persistence

Persistent:

```text
PostgreSQL
```

Potential future persistent object storage:

```text
Course assets
Images
User uploads
```

Ephemeral:

```text
Runner filesystem
Container state
Execution workspace
```

---

# 48. Security Baseline

Infrastructure harus memenuhi baseline:

```text
HTTPS
Database private
Secrets outside Git
Runner network disabled
Runner resource limits
Runner filesystem isolation
No privileged runner
No host filesystem mount
No runtime socket inside runner
Rate limiting
Request size limits
Container cleanup
Database backups
```

---

# 49. Infrastructure Decision Summary

```text
Deployment
→ Single VM initially

Backend
→ Rust modular monolith

Frontend
→ Astro

Database
→ PostgreSQL

Database topology
→ Schema per bounded context

Local container runtime
→ Podman

Production container runtime
→ TBD after deployment/security evaluation

Runner
→ Ephemeral containers

Execution
→ Synchronous

Queue
→ None initially

Network inside runner
→ Disabled

Runner filesystem
→ Ephemeral

Runner resource limits
→ CPU + memory + process + timeout

Reverse proxy
→ Yes

HTTPS
→ Yes

Database public exposure
→ No

Backups
→ Yes

Centralized logging
→ Not required initially

Metrics
→ Yes, minimal
```

---

# 50. Final MVP Topology

```text
                         INTERNET
                            │
                           HTTPS
                            │
                            ▼
                    ┌───────────────┐
                    │ Reverse Proxy │
                    └───────┬───────┘
                            │
                            ▼
                  ┌───────────────────┐
                  │   Rust API        │
                  │ Modular Monolith  │
                  └───────┬───────────┘
                          │
             ┌────────────┼────────────┐
             │            │            │
             ▼            ▼            ▼
      ┌────────────┐ ┌──────────┐ ┌──────────────┐
      │ PostgreSQL │ │ Learning │ │ Execution    │
      │            │ │ Modules  │ │ Module       │
      └────────────┘ └──────────┘ └──────┬───────┘
                                         │
                                         ▼
                                  Container Runtime
                                         │
                                         ▼
                              ┌────────────────────┐
                              │ Ephemeral Runner   │
                              │                    │
                              │ CPU limited        │
                              │ Memory limited     │
                              │ Timeout             │
                              │ No network          │
                              │ Isolated filesystem│
                              └────────────────────┘
```

Infrastructure ini cukup untuk MVP tanpa menciptakan operational complexity yang belum dibutuhkan.

Security-sensitive area tetap memiliki boundary yang jelas sehingga runner dapat dipisahkan ke host/worker architecture ketika execution workload meningkat.
