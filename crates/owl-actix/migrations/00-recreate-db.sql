-- # For PostgreSQL

-- -- FOR DEVELOPMENT ONLY - Brute force DROP DB (for local dev and unit test)
-- SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'owl_project' OR datname = 'owl_db';
-- DROP DATABASE IF EXISTS owl_db;
-- DROP USER IF EXISTS owl_project;
--
-- -- FOR DEVELOPMENT ONLY - Dev only password (for local dev and unit test)
-- CREATE USER owl_project PASSWORD 'dev_only_pwd';
-- CREATE DATABASE owl_db owner owl_project ENCODING = 'UTF-8';

-- # For MySQL

-- Terminate connections and drop database/user
-- SELECT CONCAT('KILL ', id, ';') 
-- FROM information_schema.processlist 
-- WHERE user = 'owl_dev' OR db = 'owl_db';
--
-- -- Pause for a moment to allow time for connections to be terminated
-- DO GET_LOCK('pause_for_connections', 10);

DROP DATABASE IF EXISTS owl_db;
DROP USER IF EXISTS 'owl_dev';

-- Release the lock
DO RELEASE_LOCK('pause_for_connections');

-- Create user and database
CREATE USER IF NOT EXISTS 'owl_dev' IDENTIFIED BY 'dev_only_pwd';
CREATE DATABASE IF NOT EXISTS owl_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
GRANT ALL PRIVILEGES ON owl_db.* TO 'owl_dev';
