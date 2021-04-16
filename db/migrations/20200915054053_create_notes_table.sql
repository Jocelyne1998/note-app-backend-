-- migrate:up
create table notes (
  title varchar(255) PRIMARY KEY,
  status varchar(1) NOT NULL
);

-- migrate:down
drop table notes;
