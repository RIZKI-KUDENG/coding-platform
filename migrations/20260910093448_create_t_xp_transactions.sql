-- Add migration script here
CREATE SCHEMA IF NOT EXISTS gamification;

-- Transaksi Ledger XP (Append-Only)
CREATE TABLE gamification.t_xp_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    exercise_id UUID,
    amount INT NOT NULL,
    type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_t_xp_transactions_user_exercise UNIQUE (user_id, exercise_id)
);

-- Index untuk mempercepat kalkulasi akumulasi total XP per user
CREATE INDEX idx_t_xp_transactions_user_id ON gamification.t_xp_transactions(user_id);
