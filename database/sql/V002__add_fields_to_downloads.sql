ALTER TABLE downloads ADD COLUMN state varchar(40) NOT NULL;
ALTER TABLE downloads ADD COLUMN link text NOT NULL;
ALTER TABLE downloads ADD COLUMN file text NOT NULL;
ALTER TABLE downloads ADD COLUMN insert_time timestamptz NOT NULL;
