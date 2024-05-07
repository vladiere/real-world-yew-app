-- Add migration script here

create or replace
view `admin_info_details` as
select
    `ui`.`id` as `id`,
    `ui`.`firstname` as `firstname`,
    `ui`.`middlename` as `middlename`,
    `ui`.`lastname` as `lastname`,
    `ui`.`email_address` as `email_address`,
    `ul`.`username` as `username`,
    `ui`.`role_user` as `role_user`,
    `ul`.`token_salt` AS `token_salt`,
    `ui`.`recent_address` as `recent_address`,
    `ui`.`civil_status` as `civil_status`,
    `ui`.`gender` as `gender`,
    `ui`.`occupation` as `occupation`,
    `ui`.`status` as `status`,
    date_format(`ui`.`ctime`, '%M %e, %Y') as `date_enrolled`
from
    (`user_info` `ui`
left join `user_login` `ul` on
    (`ul`.`user_id` = `ui`.`id`));

create or replace
view `user_info_details` as
select
    `ui`.`id` as `id`,
    `ui`.`firstname` as `firstname`,
    `ui`.`middlename` as `middlename`,
    `ui`.`lastname` as `lastname`,
    `ui`.`email_address` as `email_address`,
    `ui`.`role_user` as `role_user`,
    `ui`.`recent_address` as `recent_address`,
    `ui`.`civil_status` as `civil_status`,
    `ui`.`gender` as `gender`,
    `ui`.`occupation` as `occupation`,
    `b`.`tower` as `tower`,
    `b`.`room` as `room`,
    `b`.`package` as `package`,
    `ui`.`status` as `status`,
    date_format(`ui`.`ctime`, '%M %e, %Y') as `date_enrolled`
from
    (`user_info` `ui`
join `building` `b` on
    (`b`.`user_id` = `ui`.`id`));
