INSERT INTO downloads (state, link, file, insert_time)
VALUES ($1, $2, $3, $4)
RETURNING download_id
