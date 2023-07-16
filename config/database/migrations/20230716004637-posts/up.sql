CREATE TABLE posts (
    id Integer NOT NULL PRIMARY KEY,
    Author VarChar(250) NULL,
    Title Text NULL,
    Content Text NULL,
    Tags Text [] NULL,
    Publish_Date Timestamp NULL,
    Status Text NULL
);