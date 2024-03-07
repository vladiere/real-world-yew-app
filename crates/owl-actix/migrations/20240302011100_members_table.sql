-- Add migration script here

DROP TABLE IF EXISTS members_table;

CREATE TABLE members_table (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id INT NOT NULL,
  firstname VARCHAR(255) NOT NULL,
  middlename VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  age INT(11) NOT NULL,
  gender ENUM('Male', 'Female'),
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
