USE ECommerce;

-- Data insertion portion --


-- Customer Creation --
-- Chat GPT is really good at making these up hahah
insert into Customers (fullName, email, address, phone)
values
("Cameron", "PociskCD@mail.uc.edu", "Coy St", "5138255922"),
("Jordan", "jordan_2023@gmail.com", "Oak St", "5129876543"),
("Taylor", "taylor_smith@outlook.com", "Pine Rd", "5123456789"),
("Morgan", "morgan.white@aol.com", "Elm Dr", "5134567890"),
("Casey", "casey.brown@yahoo.com", "Birch Ln", "5136543210"),
("Jamie", "jamie.doe@icloud.com", "Cedar Way", "5122468100"),
("Reese", "reese_001@hotmail.com", "Willow Blvd", "5125551234"),
("Skyler", "skyler.green@gmail.com", "Sunset Rd", "5137776543"),
("Quinn", "quinn.jameson@protonmail.com", "River St", "5128884321"),
("Riley", "riley.jackson@msn.com", "Lakeside Ave", "5131239876");
SELECT * from Customers; 

insert into CreditCards (digits, cvv, expirationDate)
values
("4532012345678901", "123", "12-25"),
("6011512345678902", "234", "06-24"),
("5105105105105100", "345", "09-26"),
("4111111111111111", "456", "03-27"),
("377777777777777", "567", "11-25"),
("6011512345678903", "678", "08-28"),
("5432123456789012", "789", "05-29"),
("4111111111111122", "890", "04-30"),
("6011512345678904", "901", "01-31"),
("5105105105105101", "012", "02-32");
SELECT * from CreditCards;

insert into Products (name, price, description, seller)
values
("Smartphone", 299, "Latest model with 5G support", "TechStore"),
("Laptop", 799, "Powerful gaming laptop with RTX graphics", "GamerShop"),
("Bluetooth Headphones", 89, "Noise-cancelling over-ear headphones", "SoundHub"),
("Smartwatch", 159, "Fitness tracking smartwatch with heart rate monitor", "FitGear"),
("Wireless Mouse", 25, "Ergonomic design wireless mouse", "TechieShop"),
("4K TV", 599, "Ultra HD 4K LED TV", "HomeElectronics"),
("Tablet", 249, "10-inch tablet with high-resolution display", "GadgetPro"),
("Gaming Chair", 199, "Ergonomic gaming chair with lumbar support", "GameHub"),
("External Hard Drive", 99, "1TB external hard drive with USB 3.0", "StorageWorld"),
("Portable Speaker", 49, "Waterproof portable Bluetooth speaker", "SoundBlaster");
SELECT * from Products;

-- drop database ECommerce;

INSERT INTO Purchases(date, description, cuId, digits, prId)
values
('08-07-23', 'Smartphone purchase', 1, '4532012345678901', 1),
('08-08-23', 'Laptop purchase', 2, '6011512345678902', 2),
('08-09-23', 'Bluetooth Headphones purchase', 3, '5105105105105100', 3),
('08-10-23', 'Smartwatch purchase', 4, '4111111111111111', 4),
('08-11-23', 'Wireless Mouse purchase', 5, '377777777777777', 5),
('08-12-23', '4K TV purchase', 6, '6011512345678903', 6),
('08-13-23', 'Tablet purchase', 7, '5432123456789012', 7),
('08-14-23', 'Gaming Chair purchase', 8, '4111111111111122', 8),
('08-15-23', 'External Hard Drive purchase', 9, '6011512345678904', 9),
('08-16-23', 'Portable Speaker purchase', 10, '5105105105105101', 10);
SELECT * from Purchases;


