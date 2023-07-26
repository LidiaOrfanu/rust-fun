-- Add migration script here
CREATE TABLE penguin (
    id  SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    species varchar(255) NOT NULL,
    age  INTEGER
);