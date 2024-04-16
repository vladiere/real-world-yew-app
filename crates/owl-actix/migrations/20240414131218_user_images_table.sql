-- Add migration script here

create table user_images (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,

  user_id BIGINT not null,

  image_path varchar(128) not null,

  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
