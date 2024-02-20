-- Add migration script here

delimiter //
create or replace procedure create_user_or_admin(
  in p_firstname varchar(255),
  in p_middlename varchar(255),
  in p_lastname varchar(255),
  in p_email varchar(255),
  in p_username varchar(255),
  in p_password varchar(255),
  in p_pass_salt varchar(255),
  in p_token_salt varchar(255)
)
begin
  declare v_id int;

  if (select count(*) from user_login where username = p_username) = 0 then
    insert into user_info (firstname, middlename, lastname, email_address) values (p_firstname,p_middlename,p_lastname, p_email);

    set v_id = LAST_insert_ID();

    insert into user_login (user_id,username,password,pass_salt,token_salt) values (v_id,p_username,p_password,p_pass_salt,p_token_salt);
    select 'User added successfully' as message;
    commit;
  else
    select 'Username already exist' as message;
  end if;
end //
delimiter ;

