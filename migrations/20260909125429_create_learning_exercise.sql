-- Add migration script here
CREATE TABLE learning.m_exercises (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       lesson_id UUID NOT NULL REFERENCES learning.m_lessons(id) ON DELETE CASCADE,
       slug VARCHAR(100) NOT NULL,
       title VARCHAR(255) NOT NULL,
       description TEXT NOT NULL,
       starter_code TEXT,
       language VARCHAR(50) NOT NULL,
       "order" INT NOT NULL,
       xp_reward INT NOT NULL DEFAULT 100,
       status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       CONSTRAINT uq_m_exercises_lesson_order UNIQUE (lesson_id, "order"),
       CONSTRAINT uq_m_exercises_lesson_slug UNIQUE (lesson_id, slug)
   );

CREATE TABLE learning.m_exercise_test_cases
 (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       exercise_id UUID NOT NULL REFERENCES learning.m_exercises(id) ON DELETE CASCADE,
       input TEXT NOT NULL,
       expected_output TEXT NOT NULL,
       is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
       "order" INT NOT NULL,
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       CONSTRAINT uq_m_exercise_test_cases_order UNIQUE (exercise_id, "order")
);
