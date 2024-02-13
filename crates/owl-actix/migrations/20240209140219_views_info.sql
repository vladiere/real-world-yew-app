-- Add migration script here

create or replace view admin_info as
select
  ui.id,
  ui.firstname,
  ui.middlename,
  ui.lastname,

from
  user_info ui,
left join user_login ul on ul.user_id = ui.id
left join user_job uj on uj.user_id = ui.id
left join user_contact_info uci on uci.user_id = ui.id;
    pub tower: String,
    pub occupation: String,
    pub position: String,
    pub email_address: String,
    pub contact_number: String,
    pub username: String,
