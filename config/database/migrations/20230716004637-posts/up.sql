CREATE TABLE Posts (
    id SERIAL NOT NULL PRIMARY KEY,
    Author VarChar(250) NULL,
    Title Text NULL,
    Content Text NULL,
    Tags Text [] NULL,
    Publish_Date Timestamp NULL,
    Status Text NULL,
    Image_Url Text NULL,
    Category Text NULL,
    Created_At Timestamp NULL,
    Updated_At Timestamp NULL,
    Slug Text NULL
);