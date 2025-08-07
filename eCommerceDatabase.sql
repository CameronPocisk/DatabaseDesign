-- This file defines the database, and its tables (relations)
-- drop database ECommerce; 
CREATE database IF NOT EXISTS ECommerce;
USE ECommerce;

CREATE table Customers(
	cuId int not NULL AUTO_INCREMENT primary key, -- Using mySql to my advantage
    fullName VARCHAR(50),
    email VARCHAR(50),
    address VARCHAR (50),
    phone VARCHAR(10)
);

CREATE table CreditCards(
	digits VARCHAR(16) primary key,
    cvv VARCHAR(3), -- cvv is the three didgits
    expirationDate VARCHAR(5) -- 06/28
);

CREATE table Products(
	prId int not NULL AUTO_INCREMENT primary key,
    name VARCHAR(100),
    price int,
    description VARCHAR(200),
    seller VARCHAR(50)
);

-- RELATIONS --

-- While this is a relation, Has its own PK so same custy with same card can buy same thing
CREATE table Purchases(
	puId int not NULL AUTO_INCREMENT primary key,
    date VARCHAR(8),
    description VARCHAR(200),
    -- Fields for foreign key
    cuId int, -- Customer
    digits VARCHAR(16), -- CreditCard
    prId int, -- Product
    FOREIGN KEY (cuId) REFERENCES Customers(cuId),
    FOREIGN KEY (digits) REFERENCES CreditCards(digits),
    FOREIGN KEY (prId) REFERENCES Products(prId)
);


-- This table is for the m->m relation between Customers and Credit Cards
CREATE table PaysWith(
	cuId int,
    digits VARCHAR(16),
    primary key(cuId, digits),
    FOREIGN KEY (cuId) REFERENCES Customers(cuId),
    FOREIGN KEY (digits) REFERENCES CreditCards(digits)
);
