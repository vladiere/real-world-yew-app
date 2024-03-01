-- Add migration script here

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
  role_user ENUM('Admin', 'Super', 'User') NOT NULL DEFAULT 'Admin',
  firstname VARCHAR(255) NOT NULL,
  middlename VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  email_address VARCHAR(255) NOT NULL UNIQUE,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_login table
CREATE TABLE user_login (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  username VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  pass_salt VARCHAR(255) NOT NULL,
  token_salt VARCHAR(255) NOT NULL,
  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create building table
CREATE TABLE building (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  user_id BIGINT NOT NULL,
  tower VARCHAR(255) NOT NULL,
  room VARCHAR(255) NOT NULL,
  pack VARCHAR(255) NOT NULL,
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

