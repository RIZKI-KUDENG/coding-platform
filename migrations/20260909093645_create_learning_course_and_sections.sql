-- Add migration script here
CREATE SCHEMA IF NOT EXISTS learning;

-- Master data courses
CREATE TABLE learning.m_courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Master data sections (Bab)
CREATE TABLE learning.m_sections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES learning.m_courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    "order" INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_m_sections_course_order UNIQUE (course_id, "order")
);
