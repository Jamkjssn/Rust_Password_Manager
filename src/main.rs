use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};

// Start by creating our LoginInfo struct
struct LoginInfo {
    website: String,
    username: String,
    password: String,
}

fn main() {
    clear();
    // Welcome them to the program
    println!("Welcome to the password manager!");
    get_input("\nPress \"Enter\" to begin");

    // Setup the vector to store all the logins 
    let mut logins: Vec<LoginInfo> = Vec::new();

    // Begin main loop
    loop{
        // Clear screen
        clear();
        // Display menu
        println!("\nPassword Manager Menu:");
        println!("1. Add a new password");
        println!("2. View a password");
        println!("3. Edit a password");
        println!("4. Delete a password");
        println!("5. Exit");

        let user_choice = get_input("Enter your choice: ");

        clear();
        match user_choice.trim() {
            // Store a new password
            "1" =>{
                add_password(&mut logins);
                get_input("\nPress \"Enter\" to return to the menu");
            }
            
            // View a password
            "2" =>{
                if logins.is_empty(){
                    println!("No passwords stored.");
                }
                else{
                    get_password(&mut logins);     
                }
                get_input("\nPress \"Enter\" to return to the menu");
            }
            
            // Edit a password
            "3" =>{
                edit_password(&mut logins);             
                get_input("\nPress \"Enter\" to return to the menu");
            }
            
            // Delete a password
            "4" =>{
                delete_password(&mut logins);
                get_input("\nPress \"Enter\" to return to the menu");
            }
            
            // Exit the program
            "5" =>{
                println!("Thank you for using the password manager!");
                get_input("\nPress \"Enter\" to finish closing the program");
                clear();
                break;
            }

            // Invalid choice
            _ =>{
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}

// Function for adding a password to the logins vector. 
fn add_password(logins: &mut Vec<LoginInfo>){
    clear();
    // First get the website name
    println!("What website will be using this password?");
    let website = get_input("Website Name: ");
    
    clear();
    // Prompt user for username and password
    println!("Enter the Username and Password for \"{website}\" below.");
    let username = get_input("Enter your Username: ");
    let password = get_input("Enter your Password: ");
    
    // Create a new LoginInfo
    let new_login = LoginInfo {website, username, password};
    
    // Push new LoginInfo to the vector
    logins.push(new_login);
}

// Function for retrieving a password from the password vector.
fn get_password(logins: &mut Vec<LoginInfo>){
    // First display the logins to the user
    display_logins(&logins);
    // Prompt user for website associated with the password
    println!("\nEnter the name of the website you want to view the password for.");
    let website = get_input("Website Name: ");
    clear();
    match find_by_name(logins, &website){
        Some(login) => {
            println!("\nLogin info for \"{}\":", website);
            println!("Username: {}", login.username);
            println!("Password: {}", login.password);
        }
        None => {
            println!("Check the website name and try again.");
        }
    }
}

// Function for editing contents of a password in the password vector.
fn edit_password(logins: &mut Vec<LoginInfo>){
    clear();
    // First display the logins to the user
    display_logins(&logins);
    // Prompt user for website associated with the password
    println!("\nEnter the name of the website you want to edit the password for.");
    let website = get_input("Website Name: ");
    clear();
    match find_by_name(logins, &website){
        Some(login) => {
            // First display current username and password
            println!("Here is the login info currently stored for {}:", website);
            println!("Username: {}\nPassword: {}", login.username, login.password);

            // Prompt user for updated username and password
            println!("\nEnter the new associated Username and Password.");
            let new_username = get_input("Enter the updated Username: ");
            let new_password = get_input("Enter the updated Password: ");

            // Update username and password
            login.username = new_username;
            login.password = new_password;
        }
        None => {
            println!("Check the website name and try again.");
        }  
    }
}

// Function for deleting a password from the password vector.
fn delete_password(logins: &mut Vec<LoginInfo>){
    // First display the logins to the user
    display_logins(&logins);
    // Prompt user for website associated with the password
    println!("\nEnter the name of the website you want to delete the password for.");
    let website = get_input("Website Name: ");
    match find_by_name(logins, &website){
        Some(_login) => {
            clear();
            // Confirm with user before deleting
            println!("Are you sure you want to delete the login info for {}?", website);
            let confirm = get_input("Enter 'yes' to confirm: ");
            if confirm == "yes"{
                // Remove the login from the vector
                logins.retain(|login| login.website != website);
            }
            else {
                // Anything but 'yes' will cancel the deletion
                println!("Deletion cancelled");
            }
        }
        None => {
            println!("Check the website name and try again.");
        }
    }
}

// Function for displaying a list of all of the websites we already have a password for
fn display_logins(logins: &Vec<LoginInfo>){
    if logins.len() == 0{
        println!("No passwords stored.")
    }
    println!("Currently saved logins:");
    let mut index = 1;
    for login in logins{
        println!("\t{}. {}", index, login.website);
        index += 1;
    }
}

// This function will be used in a couple of the other functions, it takes in our vector of Logins and the name of the login we're looking for, returns the login struct if its in the vector
fn find_by_name<'a>(logins: &'a mut Vec<LoginInfo>, name: &str) -> Option<&'a mut LoginInfo>{
    // Iterate over the vector of logins searching for a matching name
    for login in logins {
        if login.website == name {
            // Login has been found, return it
            return Some(login);
        }
    }
    // No login was found
    None
}

// Creating this password to simplify the process of getting an input down to something more familiar and intuitive
fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
    }

// Simply clears the terminal
// Disclaimer: This was copied from ChatGPT
fn clear(){
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    stdout().flush().unwrap();
}