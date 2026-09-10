-- Add migration script here
CREATE SCHEMA IF NOT EXISTS system;


CREATE TABLE system.m_feature_flags (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       key VARCHAR(100) UNIQUE NOT NULL,       --
       is_enabled INT NOT NULL DEFAULT 1,
       description TEXT,                         --
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
   );
