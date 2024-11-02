use inquire::Select;
// use std::fs;
// use std::process::{Command, exit};
use std::process:: exit;
use git2::Repository;

fn main() {
    println!("Welcome to the Account Abstraction Project Bootstrapper!");

    let options = vec!["Foundry Non-Diamond", "Foundry Diamond"];
    println!(" ");
    let framework = Select::new("Choose your Project Type:", options)
        .prompt()
        .expect("Failed to read input");

    match framework {
        "Foundry Non-Diamond" => setup_foundry(),
        "Foundry Diamond" => setup_foundry_diamond(),
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}

fn setup_foundry() {
    println!("Setting up an Account Abstraction project using Foundry (Non Diamond Implementation)...");

    let repo_url = "https://github.com/web3normad/AccountAbstraction-test.git";
    let local_path = "./AccountAbstraction-test";

    match Repository::clone(repo_url, local_path) {
        Ok(_) => { 
            println!("Successfully bootstrap Account-Abstraction !");
            println!("");
            println!("=========================");
           
            println!("cd {}", local_path);
        println!("cd ./src");
            println!("Run forge build .....");
            println!("=========================");
        }
        ,
        Err(e) => eprintln!("Failed to clone repository: {}", e),
    }
}

fn setup_foundry_diamond() {
    println!("Setting up an Account Abstraction project using Foundry (Diamond Implementation)...");

    println!("Hardhat setup completed!");
}

