-- Migration to seed feature flags for learning lessons, exercises, and submissions
-- Ensure parent courses flag exists and get its ID
DO $$
DECLARE
    v_courses_id UUID;
    v_lessons_id UUID;
    v_exercise_id UUID;
BEGIN
    SELECT id INTO v_courses_id FROM system.m_feature_flags WHERE key = 'courses';

    -- 1. courses:lessons (sub-fitur dari courses)
    INSERT INTO system.m_feature_flags (id, key, is_enabled, description, parent_id, created_at, updated_at)
    VALUES (
        gen_random_uuid(),
        'courses:lessons',
        1,
        'Modul materi pelajaran dalam alur kursus',
        v_courses_id,
        NOW(),
        NOW()
    )
    ON CONFLICT (key) DO UPDATE
    SET parent_id = EXCLUDED.parent_id,
        description = EXCLUDED.description,
        updated_at = NOW()
    RETURNING id INTO v_lessons_id;

    -- If already existed, fetch v_lessons_id
    IF v_lessons_id IS NULL THEN
        SELECT id INTO v_lessons_id FROM system.m_feature_flags WHERE key = 'courses:lessons';
    END IF;

    -- 2. courses:exercise (sub-fitur dari courses:lessons)
    INSERT INTO system.m_feature_flags (id, key, is_enabled, description, parent_id, created_at, updated_at)
    VALUES (
        gen_random_uuid(),
        'courses:exercise',
        1,
        'Modul latihan koding dan test case pada pelajaran',
        v_lessons_id,
        NOW(),
        NOW()
    )
    ON CONFLICT (key) DO UPDATE
    SET parent_id = EXCLUDED.parent_id,
        description = EXCLUDED.description,
        updated_at = NOW()
    RETURNING id INTO v_exercise_id;

    -- If already existed, fetch v_exercise_id
    IF v_exercise_id IS NULL THEN
        SELECT id INTO v_exercise_id FROM system.m_feature_flags WHERE key = 'courses:exercise';
    END IF;

    -- 3. courses:submission (sub-fitur dari courses:exercise)
    INSERT INTO system.m_feature_flags (id, key, is_enabled, description, parent_id, created_at, updated_at)
    VALUES (
        gen_random_uuid(),
        'courses:submission',
        1,
        'Layanan pengiriman solusi dan penilaian otomatis biner test cases',
        v_exercise_id,
        NOW(),
        NOW()
    )
    ON CONFLICT (key) DO UPDATE
    SET parent_id = EXCLUDED.parent_id,
        description = EXCLUDED.description,
        updated_at = NOW();

END $$;
