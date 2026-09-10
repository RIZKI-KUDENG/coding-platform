-- Add migration script here

-- Master data lessons (Materi Pembelajaran)
CREATE TABLE learning.m_lessons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Master relasi section dan lesson (Reusable Lessons)
CREATE TABLE learning.m_section_lessons (
    section_id UUID NOT NULL REFERENCES learning.m_sections(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL REFERENCES learning.m_lessons(id) ON DELETE CASCADE,
    "order" INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (section_id, lesson_id),
    CONSTRAINT uq_m_section_lessons_order UNIQUE (section_id, "order")
);
