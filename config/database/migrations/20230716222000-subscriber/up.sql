CREATE TABLE subscriber (
    id Integer SERIAL NOT NULL PRIMARY KEY UNIQUE,
    email Text NOT NULL UNIQUE
);