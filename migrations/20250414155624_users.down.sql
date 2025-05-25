-- Add down migration script here
drop table if exists "user";
drop type if exists user_role;
drop extension if exists "uuid-ossp";