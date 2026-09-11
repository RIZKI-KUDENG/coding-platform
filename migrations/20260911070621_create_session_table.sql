-- Add migration script here
CREATE TABLE identity.sessions(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE INDEX idx_session_user_id
ON identity.sessions(user_id);

CREATE INDEX idx_session_expires_at
ON identity.sessions(expires_at);
