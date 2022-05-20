UPDATE downloads
SET state = $1,
    file  = $2
WHERE download_id = $3
