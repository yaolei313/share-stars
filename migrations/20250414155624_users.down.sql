-- Add down migration script here
drop table if exists "users";
drop type if exists user_role;
drop extension if exists "uuid-ossp";