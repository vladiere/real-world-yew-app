-- Add migration script here

create table device_info (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,

  device_name varchar(128) unique not null,
  device_tower varchar(128) not null,
  device_room varchar(128) not null,
  device_state enum('Standby', 'Working', 'Repair', 'Unavailable', 'Available', 'Damaged') not null default 'Standby',

  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
