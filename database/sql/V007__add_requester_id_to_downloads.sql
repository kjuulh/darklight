ALTER TABLE downloads ADD COLUMN requester_id UUID default gen_random_uuid() not null;

CREATE INDEX CONCURRENTLY download_requester_id_idx ON downloads (requester_id)
