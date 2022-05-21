ALTER TABLE downloads ADD COLUMN download_id UUID NOT NULL DEFAULT gen_random_uuid();
