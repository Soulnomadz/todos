-- Add migration script here
create table if not exists users (
  id serial primary key,
  name varchar(50) not null unique,
  pass varchar(255) not null,
  comment varchar(255) null,
  created_at timestamp with time zone not null default current_timestamp,
  updated_at timestamp with time zone not null default current_timestamp
);

create or replace function update_users_updated_at() 
returns trigger as $$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$ language plpgsql;

create trigger trigger_users_updated_at
before update on users
for each row
execute function update_users_updated_at();

