CREATE TABLE jobs (
    job_id UUID PRIMARY KEY,
    job_type VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    attempts INTEGER NOT NULL,
    max_attempts INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE,
    finished_at TIMESTAMP WITH TIME ZONE,
    error TEXT,
    payload JSONB,
    result JSONB
);
