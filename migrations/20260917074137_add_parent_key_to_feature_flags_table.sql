-- Add migration script here
ALTER TABLE system.m_feature_flags
ADD COLUMN parent_id UUID
REFERENCES system.m_feature_flags(id) ON DELETE CASCADE;
