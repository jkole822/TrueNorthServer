ALTER TABLE decisions
ADD COLUMN category TEXT,
ADD COLUMN emotions TEXT[],
ADD COLUMN desired_outcome TEXT;