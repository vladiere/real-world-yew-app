-- Add migration script here

create table monitoring_table (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,

  client_name varchar(128) not null,
  building_tower varchar(128) not null,
  building_room varchar(128) not null,
  monitor_state enum('Opened', 'Closed'),

  ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  mtime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
