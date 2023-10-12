CREATE TABLE IF NOT EXISTS score (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    total INTEGER NOT NULL,
    board_id UUID REFERENCES board(id) ON DELETE CASCADE,
    player TEXT NOT NULL
);