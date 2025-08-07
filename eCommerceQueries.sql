-- Project and MISC Queries --

-- Write at least one multi-table query. Example: Show 
-- the names of customers along with the names of the products they purchased 
-- where product price > $100
select Customers.fullName
from Purchases
left join Customers on Purchases.cuId=Customers.cuId
left join Products on Purchases.prId=Products.prId
group by Customers.fullName
having SUM(Products.price) > 100;

-- select * from Customers;select * from Purchases, Customers, Products left join Customers on Purchases.cuId=Customers.cuId LIMIT 0, 1000
