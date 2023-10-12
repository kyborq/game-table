CREATE TABLE IF NOT EXISTS board (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    game_id UUID REFERENCES game(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);