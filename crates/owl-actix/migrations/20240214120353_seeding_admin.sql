-- Add migration script here
CALL owl_db.create_user_or_admin("christian", "guapo", "perez", "b1612", "programmer", "senior", "kulas@email.com", "+639874562315", "kulas.admin", "$argon2id$v=19$m=4096,t=192,p=4$0ycbcnsu+NhwrBpDQbGND9Yb2NXdfq2o/UmdNUmYTfs$+7yFPfeN+ZOpsD6QRS6tUf+pfJv6LKh9e1RByuEZdNA", "c66f76c2-26df-4182-be7c-99d996ee1476", "c66f76c2-26df-4182-be7c-99d996ee1476"); 
