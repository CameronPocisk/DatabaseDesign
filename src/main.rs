// I am not sure how to use rust. Going to learn some type shi


// I thik i need to use the carge package manager / build tool
use mysql::{prelude::Queryable, *}; // for connecting to mySQL
use std::io::{self, Write};
use std::string;
use mysql::Result; // Error handling
use mysql::prelude::*; // Get some more traits from the crate


fn AddOption(conn: &mut PooledConn) -> Result<()>{
    println!("What would you like to add? ");
    println!("1. Customer, 2. Credit Card, 3. Product, 4. Purchase, 5. Pays With Relation, 6. Cancel");

    let mut option = String::new();
    std::io::stdin().read_line(&mut option)?;
    option = option.trim().to_string();

    if option == "1" {

        // Get the variables that we need
        print!("Enter the full name: ");
        io::stdout().flush()?; // Flush the output so this can be one line
        let mut fullName = String::new();
        std::io::stdin().read_line(&mut fullName)?;
        fullName = fullName.trim().to_string();

        print!("Enter the email: ");
        io::stdout().flush()?;
        let mut email = String::new();
        std::io::stdin().read_line(&mut email)?;
        email = email.trim().to_string();

        print!("Enter the address: ");
        io::stdout().flush()?;
        let mut address = String::new();
        std::io::stdin().read_line(&mut address)?;
        address = address.trim().to_string();
        

        print!("Enter the phone number (10 digits): ");
        io::stdout().flush()?;
        let mut phoneNumber = String::new();
        std::io::stdin().read_line(&mut phoneNumber)?;
        phoneNumber = phoneNumber.trim().to_string();


        conn.exec_drop(
        "INSERT INTO Customers (fullName, email, address, phone) VALUES (?, ?, ?, ?)",
        (fullName, email, address, phoneNumber),
        )?;

        print!("Inserted into Customers Table");
    }
    else if  option == "2"{
        // Credit card
        print!("Enter the 16 digits: ");
        io::stdout().flush()?;
        let mut digits = String::new();
        std::io::stdin().read_line(&mut digits)?;
        digits = digits.trim().to_string();

        print!("Enter the cvv (3 numbers): ");
        io::stdout().flush()?;
        let mut cvv = String::new();
        std::io::stdin().read_line(&mut cvv)?;
        cvv = cvv.trim().to_string();

        print!("Enter the Exp Dste(MM-YY): ");
        io::stdout().flush()?;
        let mut expDate = String::new();
        std::io::stdin().read_line(&mut expDate)?;
        expDate = expDate.trim().to_string();

                conn.exec_drop(
        "INSERT INTO Credit Cards (fullNamedigits, emailcvv, expirationDate) VALUES (?, ?, ?)",
        (digits, cvv, expDate),
        )?;

        print!("Inserted into Customers Table");
    }
    else if  option == "3"{
        // Product
        print!("Enter the product name: ");
        io::stdout().flush()?;
        let mut name = String::new();
        std::io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();

        print!("Enter the price: ");
        io::stdout().flush()?;
        let mut price = String::new();
        std::io::stdin().read_line(&mut price)?;
        price = price.trim().to_string();

        print!("Enter the description: ");
        io::stdout().flush()?;
        let mut description = String::new();
        std::io::stdin().read_line(&mut description)?;
        description = description.trim().to_string();

        print!("Enter the Seller: ");
        io::stdout().flush()?;
        let mut seller = String::new();
        std::io::stdin().read_line(&mut seller)?;
        seller = seller.trim().to_string();

        conn.exec_drop(
        "insert into Products (name, price, description, seller) VALUES(?, ?, ?, ?)",
        (name, price, description, seller),
        )?;

    }
    else if  option == "4"{
        // Purchases
        print!("Enter the purchase date: ");
        io::stdout().flush()?;
        let mut date = String::new();
        std::io::stdin().read_line(&mut date)?;
        date = date.trim().to_string();

        print!("Enter the description: ");
        io::stdout().flush()?;
        let mut description = String::new();
        std::io::stdin().read_line(&mut description)?;
        description = description.trim().to_string();

        print!("Enter Customer Id: ");
        io::stdout().flush()?;
        let mut cuId = String::new();
        std::io::stdin().read_line(&mut cuId)?;
        cuId = cuId.trim().to_string();

        print!("Enter the Credit Card digits: ");
        io::stdout().flush()?;
        let mut digits = String::new();
        std::io::stdin().read_line(&mut digits)?;
        digits = digits.trim().to_string();

        print!("Enter the Product Id: ");
        io::stdout().flush()?;
        let mut prId = String::new();
        std::io::stdin().read_line(&mut prId)?;
        prId = prId.trim().to_string();

        conn.exec_drop(
        "INSERT INTO Purchases(date, description, cuId, digits, prId) VALUES(?, ?, ?, ?, ?)",
        (date, description, cuId, digits, prId),
        )?;
    }
    else if  option == "5"{
        print!("Enter the Card Owner: ");
        io::stdout().flush()?;
        let mut cardOwner = String::new();
        std::io::stdin().read_line(&mut cardOwner)?;
        cardOwner = cardOwner.trim().to_string();

        print!("Enter the Credit Card digits: ");
        io::stdout().flush()?;
        let mut digits = String::new();
        std::io::stdin().read_line(&mut digits)?;
        digits = digits.trim().to_string();
        
        conn.exec_drop(
            "INSERT INTO PaysWith(cuId, digits) VALUES(?, ?)",
            (cardOwner, digits),
            )?;
    }
    else{
        print!("Canceled.");
    }

    Ok(())
}

fn ViewOption(conn: &mut PooledConn) -> Result<()>{
    println!("What table would you like to View? ");
    println!("1. Customers, 2. CreditCards, 3. Products, 4. Purchases, 5. PaysWith Relation, 6. Cancel");

    let mut option = String::new();
    std::io::stdin().read_line(&mut option)?;
    option = option.trim().to_string();

    if option == "1"{
        // Get a vector of customer tuples
        let customers: Vec<(u32, String, String, String, String)> = conn.query(
            "SELECT cuId, fullName, email, address, phone FROM Customers",
        )?;

        // Print results
        for (id, full_name, email, address, phone) in customers {
            println!(
                "ID: {}, Name: {}, Email: {}, Address: {}, Phone: {}",
                id, full_name, email, address, phone
            );
        }
    }
    else if option == "2"{

        let creditCards: Vec<(String, String, String)> = conn.query(
            "SELECT digits, fullNamecvv, expirationDate FROM CreditCards",
        )?;

        // Print results
        for (digits, cvv, date) in creditCards {
            println!(
                "CR Digits: {}, cvv: {}, exp Date: {}",
                digits, cvv, date
            );
        }
    }
    else if option == "3"{
        let Products: Vec<(u32, String, u32, String, String)> = conn.query(
            "SELECT prId, name, price, description, seller FROM Products",
        )?;

        // Print results
        for (prId, name, price, description, seller) in Products {
            println!(
                "ID: {}, name: {}, price: {}, description: {}, seller: {}",
                prId, name, price, description, seller
            );
        }
    }
    else if option == "4"{
        let Purchases: Vec<(u32, String, String, u32, String, u32)> = conn.query(
            "SELECT puId, date, description, cuId, digits, prId FROM Purchases",
        )?;

        // Print results
        for (puId, date, description, cuId, digits, prId) in Purchases {
            println!(
                "puId: {}, date: {}, description: {}, cuId: {}, digits: {}, prId: {}",
                puId, date, description, cuId, digits, prId
            );
        }
    }
    else if option == "5"{
        let PaysWith: Vec<(u32, String)> = conn.query(
            "SELECT cuId, digits FROM PaysWith",
        )?;

        // Print results
        for (cuId, digits) in PaysWith {
            println!(
                "cuId: {}, digits: {}",
                cuId, digits
            );
        }
    }
    else{
        print!("Canceled.");
    }

    Ok(())
}

fn DirectOption(conn: &mut PooledConn) -> Result<()>{

    println!("Enter the queary you would like to directly input: ");

    let mut command = String::new();
    std::io::stdin().read_line(&mut command)?;
    command = command.trim().to_string();


    conn.exec_drop(command, ())?;

    Ok(())
}

fn main() -> Result<()>{
    // make sure our shi work 
    println!("Hello, world!");

    // Setup the connection
    let url = "mysql://rustuser:1234@localhost:3306/ECommerce";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Example Queary (Seen in DB)
    // conn.query_drop(
    //     "INSERT INTO Customers(fullName, email, address, phone) \
    //     Values('Rusty', 'RustyRS@mail.uc.edu', 'Coy St', '5138255922');")?;
    
    // CLI Related things
    
    loop { // How rust wants you to do a while loop
        println!("Would you like to 1. Add, 2. View a table, 3. Directly Type a queary, or 4. quit");
        
        let mut option = String::new();
        std::io::stdin().read_line(&mut option)?;
        option = option.trim().to_string();
        
        if option == "1" {
            print!("Adding: ");
            AddOption(&mut conn);
        }
        else if option == "2" {
            print!("Viewing: ");
            ViewOption(&mut conn);
        }
        else if option == "3" {
            print!("Direct Commands: ");
            DirectOption(&mut conn);
        }
        else{
            break;
        }
    }


    
    println!("Finished program");
    Ok(())
}