-- Add migration script here

DROP TABLE IF EXISTS members_table;

CREATE OR REPLACE TABLE members_table (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  member_id VARCHAR(255) UNIQUE,

  user_id INT NOT NULL,
  firstname VARCHAR(255) NOT NULL,
  middlename VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  age INT(11) NOT NULL,
  gender ENUM('Male', 'Female'),
  status ENUM('Active','Removed', 'Inactive'),
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
