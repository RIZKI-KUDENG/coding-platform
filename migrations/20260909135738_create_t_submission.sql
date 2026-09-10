-- Add migration script here
CREATE SCHEMA IF NOT EXISTS execution;


CREATE TABLE execution.t_submissions(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    exercise_id UUID NOT NULL,
    code TEXT NOT NULL,
    language VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    execution_time_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_t_submissions_user_id ON
execution.t_submissions(user_id);
  CREATE INDEX idx_t_submissions_exercise_id
ON execution.t_submissions(exercise_id);
