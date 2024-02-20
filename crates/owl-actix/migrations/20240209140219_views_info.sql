-- Add migration script here

create or replace view user_info_details as
select
	ui.id,
	ui.firstname,
	ui.middlename,
	ui.lastname,
	ui.email_address,
	ul.username,
	ui.role_user
from
	user_info ui
left join
	user_login ul on ul.user_id = ui.id;
