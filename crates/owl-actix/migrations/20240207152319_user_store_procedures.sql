-- Add migration script here
use owl_db;

delimiter //
create or replace procedure create_user_or_admin(
  in p_firstname varchar(255),
  in p_middlename varchar(255),
  in p_lastname varchar(255),
  in p_tower varchar(255),
  in p_occupation varchar(255),
  in p_position varchar(255),
  in p_email varchar(255),
  in p_contact varchar(255),
  in p_username varchar(255),
  in p_password varchar(255),
  in p_pass_salt varchar(255),
  in p_token_salt varchar(255)
)
begin
  declare v_id int;

  if (select count(*) from user_login where username = p_username) = 0 then
    insert into user_info (firstname, middlename, lastname) values (p_firstname,p_middlename,p_lastname);

    set v_id = LAST_insert_ID();

    insert into user_login (user_id,username,password,pass_salt,token_salt) values (v_id,p_username,p_password,p_pass_salt,p_token_salt);
    insert into user_contact_info (user_id,email_address,contact_number) values (v_id,p_email,p_contact);
    insert into user_job (user_id,occupation,position) values (v_id,p_occupation,p_position);
    insert into user_picture (user_id,img_path) values (v_id, 'https://media.istockphoto.com/id/1312136351/photo/3d-illustration-of-cute-cartoon-man-with-eyeglasses-in-blue-shirt-with-arms-crossed-close-up.jpg?b=1&s=612x612&w=0&k=20&c=Ml61cAXSJuPreQqOtuYd2FVHGB1nbDN4TuqLFConXf4=');
    select 'User added successfully' as message;
    commit;
  else
    select 'Username already exist' as message;
  end if;
end //
delimiter ;

