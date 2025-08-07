# **E-Commerce System Requirements Document**

## **System Overview**

The goal of this e-commerce system is to provide a platform where customers can browse products, add multiple credit cards to their account, and make purchases. The system will manage the relationships between customers, their credit cards, the products available for sale, and their purchase history.

---

## **1. Data Requirements**

### **Customer Table:**
- **Purpose**: Stores customer account information.
- **Attributes**:
  - **Customer ID**: Unique identifier for each customer (auto-generated).
  - **Full Name**: The full name of the customer.
  - **Email**: The customer's email address (must be unique).
  - **Address**: The customer's shipping address.
  - **Phone Number**: The customer's phone number.

- **Use Case**: When a new customer registers on the system, they will provide their name, email, address, and phone number, which are stored in this table.

---

### **Product Table:**
- **Purpose**: Stores the details of products available for sale.
- **Attributes**:
  - **Product ID**: Unique identifier for each product (auto-generated).
  - **Name**: The name of the product (e.g., "Smartphone").
  - **Price**: The price of the product.
  - **Description**: A detailed description of the product.
  - **Seller**: The seller or vendor of the product.

- **Use Case**: A customer can browse available products by querying the product table, based on attributes such as name, price, and seller. Admins can add new products to the system.

---

### **Credit Card Table:**
- **Purpose**: Stores the credit card information associated with a customer.
- **Attributes**:
  - **Credit Card Number**: The credit card number.
  - **CVV**: The CVV (Card Verification Value) of the credit card.
  - **Expiration Date**: The expiration date of the credit card.
  - **Customer ID**: A reference to the customer associated with the credit card.

- **Use Case**: A customer can add multiple credit cards to their account. This allows them to select a payment method during checkout. The system should ensure that credit card data is stored securely.

---

### **Purchase Table:**
- **Purpose**: Stores details of the purchases made by customers.
- **Attributes**:
  - **Purchase ID**: Unique identifier for each purchase (auto-generated).
  - **Date**: The date when the purchase was made.
  - **Description**: A description of the purchase (e.g., "Smartphone purchase").
  - **Customer ID**: A reference to the customer who made the purchase.
  - **Credit Card Number**: A reference to the credit card used for the purchase.
  - **Product ID**: A reference to the product purchased.

- **Use Case**: When a customer makes a purchase, a new record is created in the `Purchase` table, linking the customer, credit card, and product. The purchase date and description will help identify the transaction for order processing and tracking.

---

## **2. Use Cases**

### **Use Case 1: Register a Customer**
- **Description**: A new user creates an account on the platform by providing their full name, email, address, and phone number.

- **Outcome**: A new customer is registered in the system.

---

### **Use Case 2: Browse Products**
- **Description**: A customer browses products available for purchase.

- **Outcome**: The customer can view product information retrieved from the system.

---

### **Use Case 3: Add Credit Card**
- **Description**: A customer adds one or more credit cards to their account for future purchases.
  
- **Outcome**: The credit card is added to the system and linked to the customer.

---

### **Use Case 4: Make a Purchase**
- **Description**: A customer selects a product, adds it to the cart, selects a payment method (credit card), and completes the purchase.

- **Outcome**: A new purchase is recorded, and the inventory or product stock is updated accordingly.

---

## **3. Non-Functional Requirements**

### **Scalability:**
- The system should be able to handle a large number of customers, products, and transactions simultaneously.

### **Performance:**
- The system should respond quickly to user actions, such as loading product pages or processing transactions.

### **Usability:**
- The console interface should be user-friendly, allowing customers to easily browse products, manage their accounts, and make purchases.