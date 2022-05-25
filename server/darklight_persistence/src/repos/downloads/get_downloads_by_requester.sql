SELECT *
FROM downloads
WHERE requester_id = $1
ORDER BY insert_time