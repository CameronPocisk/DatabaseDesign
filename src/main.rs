// I am not sure how to use rust. Going to learn some type shi


// I thik i need to use the carge package manager / build tool
use mysql::{prelude::Queryable, *}; // for connecting to mySQL
use std::io::{self, Write};
use mysql::Result; // Error handling


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
        "INSERT INTO Customers(fullName, email, address, phone) \
        Values(:fullName, :email, :address, :phone);",
        params! {
            "name" => fullName,
            "email" => email,
            "addr" => address,
            "phone" => phoneNumber,
        },
        )?;
        println!("Added!");
    }
    else if  option == "2"{
        
    }
    else if  option == "3"{
        
    }
    else if  option == "4"{
        
    }
    else if  option == "5"{
        
    }
    else{
        print!("Canceled.");
    }

    Ok(())
}

fn ViewOption(conn: &mut PooledConn) -> Result<()>{
    Ok(())
}

fn DirectOption(conn: &mut PooledConn) -> Result<()>{
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