use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

struct Product {
    name: String,
    description: String,
    price: i64,
    quantity: i64
}
struct Inventory {
    items: Vec<Product>
}
impl Inventory {
    fn init() -> Self {
        Inventory { items: Vec::new() }
    }
    fn add(&mut self, name: String, description: String, price: i64, quantity: i64) -> &mut Self 
    {
        let product = Product{name, description , price, quantity};
        self.items.push(product);
        self
    }

    fn edit(&mut self, name: String, new_name: String, new_description: String, new_price: i64, new_quantity: i64) -> &mut Self
    {
        if let Some(product) = self.items.iter_mut().find(|product| product.name == name) {
            product.name = new_name;
            product.description = new_description;
            product.price = new_price;
            product.quantity = new_quantity;
        }
        self
    }

    fn remove(&mut self, name: &String) -> bool
    {
        let mut found : bool = false;
        self.items.retain(|product| { 
            if(product.name != *name) {
                true
            }
            else{
                found = true;
                false
            }
    });
        if(found)
        { 
            true
        }
        else {
            false
        }
            
    }

    fn generate_report(&self) -> String
    {
        let mut buffer = String::new();

        buffer.push_str("Inventory Report\n");
        buffer.push_str("Name\tPrice\tQuantity\tDescription\n");

        for product in &self.items {
            buffer.push_str(&format!("{}\t{}\t{}\t\t{}\n", product.name, product.price, product.quantity, product.description));
        }

        buffer
    }

    fn save_report(&self, filename: &str) -> Result<(), std::io::Error>
    {
        let report = self.generate_report();

        let mut file = File::create(filename)?;

        file.write_all(report.as_bytes())?;

        Ok(())
    }
}


//-----------------------------------MENU-----------------------------------------------------------------
fn productPrint(index : i32, product : &Product)
{
    println!("({}) {} : ", index, product.name);
    println!(" Price : {} " ,product.price );
    println!(" Quantity : {} ", product.quantity);
    println!(" Description : {} ", product.description);    
}

fn printMenu ()
{
    println!("What Do you Want To Do?");
    println!("1) Add Product");
    println!("2) Edit Product");
    println!("3) View Inventory");
    println!("4) Delete Product");
    println!("5) Generate Report");
    println!("6) Save Report");
    println!("7) Log Out");
}


fn addMenu(inventory : &mut Inventory) {

    println!("Add Item : ");
    let mut input = String::new();

    println!("1) Name : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let name = input.trim().to_string();
    input.clear();
    
    println!("2) Price : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let price: i64 = input.trim().parse().expect("Invalid input");
    input.clear();
    
    println!("3) Quantity : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let quantity: i64 = input.trim().parse().expect("Invalid input");
    input.clear();
    
    println!("4) Description : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let description = input.trim().to_string();

    inventory.add(name, description, price, quantity);
    println!("Item Successfully Added!!\n\n\n");
}

fn editMenu(inventory : &mut Inventory) {

    println!("Edit Item : ");
    let mut input = String::new(); 
    
    println!("1) Name of the product to edit : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let name = input.trim().to_string();
    input.clear();
    
    println!("2) New name : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_name = input.trim().to_string();
    input.clear();
    
    println!("3) New price : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_price: i64 = input.trim().parse().expect("Invalid input");
    input.clear();
    
    println!("4) New quantity : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_quantity: i64 = input.trim().parse().expect("Invalid input");
    input.clear();
    
    println!("5) New description : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_description = input.trim().to_string();
    inventory.edit(name, new_name, new_description, new_price, new_quantity);
}


fn deleteMenu(inventory : &mut Inventory) {
    
    println!("Delete Item : ");
    let mut input = String::new(); 
    
    println!("1) Name of the product to delete : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let name = input.trim().to_string();
    
    if(inventory.remove(&name))
    {
        println!("Item Successfully Deleted! \n");
    }
    else 
    {
        println!("Item Not Found. Try Again -_- \n");
    }
    
    
}

fn generateReportMenu(inventory : &Inventory) {
    
    println!("Generate Report : ");
    println!("{}", inventory.generate_report());
}

fn saveReportMenu(inventory : &Inventory) {
    
    println!("Save Report : ");
    let mut input = String::new(); 
    
    println!("1) Enter the filename to save the report : ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let filename = input.trim();
    // save the report to the file
    match inventory.save_report(filename) {
        Ok(_) => println!("Report saved successfully."),
        Err(e) => println!("Error saving report: {}", e),
    }
}

fn logOut()
{
    println!("Logged Out!");
    std::process::exit(0);
}

fn ViewInventory(inventory : &Inventory)
{
    if inventory.items.is_empty() 
    {
        println!("The inventory is empty.");
        return;
    }
    
    println!("The inventory contains the following products:");
   
    for (index, product) in inventory.items.iter().enumerate() {
       
        productPrint(index as i32 + 1, product);
    }
}

fn optionChecker(option : i32, inventory : &mut Inventory)
{
    match option
    {
        1 => addMenu(inventory),
        2 => editMenu(inventory),
        3 => ViewInventory(&inventory),
        4 => deleteMenu(inventory),
        5 => generateReportMenu(&inventory),
        6 => saveReportMenu(&inventory),
        7 => logOut(),
        _ => println!("Wrong Command Entered!")
    }
}

// a struct to store the credentials of a store manager
struct Manager {
    username: String,
    password: String,
}

// a function to check if the given username and password match with a manager's credentials
fn authenticate(username: &str, password: &str, managers: &HashMap<String, Manager>) -> bool {

    if let Some(manager) = managers.get(username)
    {
        if manager.password == password 
        {
            return true;
        }
    }
    false
}

fn main() {

    let mut managers = HashMap::new();
    managers.insert("alice".to_string(), Manager{username: "alice".to_string(), password: "1234".to_string()});
    managers.insert("bob".to_string(), Manager{username: "bob".to_string(), password: "5678".to_string()});

    
    let mut inventory = Inventory::init();

    let mut logged_in = false;

    while !logged_in  {
    
        let mut input = String::new();

        println!("Enter your username: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let username = input.trim().to_string();
        input.clear();

        println!("Enter your password: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let password = input.trim().to_string();
        input.clear();

        if authenticate(&username, &password, &managers) 
        {
            logged_in = true;
        }
    }

    while logged_in 
    {
        printMenu();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
 
        let option:  i32 = input.trim().parse().unwrap();
        optionChecker(option, &mut inventory);
        input.clear();
    }


    
}