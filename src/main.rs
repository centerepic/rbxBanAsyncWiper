use std::fs;
use std::path::Path;
use colored::Colorize;

fn main() {

    println!("{}", "[?] rbxBanAsyncWiper v1.0.0 | sashaa169".blue());
    println!("{}", "[!] Ensure you are logged out of the banned account and that it isn't on your account management list before running this program!".yellow());

    let user = std::env::var("USERPROFILE").unwrap();
    let path = Path::new(&user).join("AppData").join("Local").join("Roblox").join("LocalStorage").join("RobloxCookies.dat");

    if !path.exists() {
        println!("{}", "[!!!] Roblox cookie file not found!".red());
        println!("[...] Press any key to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
        std::process::exit(1);
    }

    let _ = fs::remove_file(&path);

    println!("{}", "[√] Roblox cookie file has been deleted!".green());
    
    println!("[...] Press any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());

    std::process::exit(0);
}