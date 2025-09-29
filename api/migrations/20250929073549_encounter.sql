CREATE TABLE IF NOT EXISTS encounters (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    entities jsonb NOT NULL,
    active smallint NOT NULL
);
