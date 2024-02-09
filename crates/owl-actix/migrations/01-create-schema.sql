-- Base app schema

-- # For PostgreSQL
-- -- User
-- CREATE TYPE user_type AS ENUM ('Admin', 'Super', 'User');
--
-- CREATE TABLE user_info (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   role_user user_type NOT NULL DEFAULT 'User',
--
--   -- User information
--   firstname VARCHAR(255) NOT NULL,
--   middlename VARCHAR(255) NOT NULL,
--   lastname VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE user_contact_info (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   user_id BIGINT NOT NULL,
--   email_address VARCHAR(255) NOT NULL UNIQUE,
--   contact_number VARCHAR(255) NOT NULL UNIQUE,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE user_job (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   user_id BIGINT NOT NULL,
--   occupation VARCHAR(255) NOT NULL,
--   position VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE user_picture (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   user_id BIGINT NOT NULL,
--   img_path VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE user_login (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--
--   -- FK
--   user_id BIGINT NOT NULL,
--
--   -- Auth
--   username VARCHAR(255) NOT NULL UNIQUE,
--   password VARCHAR(255) NOT NULL,
--   pass_salt uuid NOT NULL DEFAULT gen_random_uuid(),
--   token_salt uuid NOT NULL DEFAULT gen_random_uuid(),
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE building (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   user_id BIGINT NOT NULL,
--   tower VARCHAR(255) NOT NULL,
--   unit VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE refresh_token (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   username VARCHAR(255) NOT NULL,
--   refresh_token VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW(),
--   mid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   mtime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- CREATE TABLE feedbacks (
--   id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
--   name VARCHAR(255) NOT NULL,
--   message VARCHAR(255) NOT NULL,
--
--   -- Timestamps
--   cid BIGINT NOT NULL GENERATED BY DEFAULT AS IDENTITY (START WITH 1000),
--   ctime timestamp with time zone NOT NULL DEFAULT NOW()
-- );
--
-- Assuming your timestamp column is named 'ctime' in the 'user_login' table
-- SELECT
--   TO_CHAR(ctime, 'YYYY-MM-DD') AS date_only,
--   TO_CHAR(ctime, 'HH24:MI:SS') AS time_only,
--   TO_CHAR(ctime, 'YYYY-MM-DD HH24:MI:SS') AS date_and_time
-- FROM user_login;

-- # For MySQL
-- Base app schema

-- User
-- CREATE ENUM user_type ('Admin', 'Super', 'User');

use owl_db;

-- Drop tables if they exist
DROP TABLE IF EXISTS user_info;
DROP TABLE IF EXISTS user_contact_info;
DROP TABLE IF EXISTS user_job;
DROP TABLE IF EXISTS user_picture;
DROP TABLE IF EXISTS user_login;
DROP TABLE IF EXISTS building;
DROP TABLE IF EXISTS refresh_token;
DROP TABLE IF EXISTS feedbacks;

-- Create user_info table
CREATE TABLE user_info (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  role_user ENUM('Admin', 'Super', 'User') NOT NULL DEFAULT 'User',
  firstname VARCHAR(255) NOT NULL,
  middlename VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_contact_info table
CREATE TABLE user_contact_info (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  email_address VARCHAR(255) NOT NULL UNIQUE,
  contact_number VARCHAR(255) NOT NULL UNIQUE,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_job table
CREATE TABLE user_job (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  occupation VARCHAR(255) NOT NULL,
  position VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_picture table
CREATE TABLE user_picture (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  img_path VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_login table
CREATE TABLE user_login (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  username VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  pass_salt VARCHAR(255) NOT NULL UNIQUE,
  token_salt VARCHAR(255) NOT NULL UNIQUE,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create building table
CREATE TABLE building (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  tower VARCHAR(255) NOT NULL,
  unit VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create refresh_token table
CREATE TABLE refresh_token (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  refresh_token VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create feedbacks table
CREATE TABLE feedbacks (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  message VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
