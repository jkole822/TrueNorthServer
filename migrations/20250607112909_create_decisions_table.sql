CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE decisions (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   answer TEXT,
   question TEXT NOT NULL,
   user_id UUID REFERENCES users(id) ON DELETE CASCADE,
   created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);