### Distributed Job Queue (Workers + Scheduler)

Job Types:
- Email
- Image
- Cleanup
- Data Processing
- Report Generation


Job Status:
- Pending
- Running
- Completed
- Failed
- Canceled

🎯 1. Project Goals
Build a production-quality distributed job queue system with:

Producers submitting jobs
- A scheduler coordinating tasks
- Multiple worker nodes processing jobs
- Visibility timeouts
- Retries with exponential backoff
- Dead-letter queue
- Web dashboard with metrics
- Horizontal scaling (add more workers anytime)

This demonstrates:
- Async Rust mastery (Tokio)
- Concurrency/channel patterns
- Networking & protocol design
- Distributed system concepts
- Clean architecture
- Testing + load/benchmarking
- Docker + Kubernetes deployment

🧱 2. System Components
FerrisQueue consists of these services:
1. API Gateway (Rust — Axum)
Accepts job submissions:
POST /jobs
GET /jobs/:id/status
Publishes jobs to Redis / Postgres / your own message log.

2. Scheduler (Rust — Tokio Task)
Responsible for:
Pulling pending jobs from storage
Assigning jobs to workers
Ensuring visibility timeouts
(“if worker dies, job reappears”)

Updating job state machine

3. Worker Nodes (Rust Binary)
Runs on multiple machines/pods:
Polls for jobs
Executes user-defined tasks (plugins or hardcoded)
Sends ACK, FAIL, or TIMEOUT
Processes in parallel using:
Tokio tasks
Semaphore for concurrency limits

4. Storage Layer
Use any of these:
PostgreSQL (job table + events)
Redis (fast queues + visibility keys)
Custom append-only log (advanced)
Use Redis at first → evolve later.

5. Dashboard (Axum + HTMX or React)
Shows:
Queue length
Worker health
Errors
Throughput
Average job latency
