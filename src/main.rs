use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::env;
use base64::{Engine as _, engine::general_purpose};

fn generate_unique_flag(employee_password: &str) -> String {
    let encoded = general_purpose::STANDARD.encode(employee_password);
    format!("FLAG{{{}}}", encoded)
}

fn main() -> io::Result<()> {
    println!("Welcome to XYZ Corporation's IT System!");
    println!("As a new employee, you need to complete this security challenge.");

    println!("Please enter your employee password:");
    let mut employee_password = String::new();
    io::stdin().read_line(&mut employee_password)?;
    let employee_password = employee_password.trim().to_string();

    let unique_flag = generate_unique_flag(&employee_password);

    let home_dir = env::var("HOME").expect("Couldn't find HOME directory");
    let base_dir = PathBuf::from(home_dir).join("xyz_corp_challenge");

    fs::create_dir_all(&base_dir)?;
    fs::create_dir_all(base_dir.join("documents"))?;
    fs::create_dir_all(base_dir.join("projects"))?;
    fs::create_dir_all(base_dir.join("backups"))?;

    File::create(base_dir.join("welcome.txt"))?.write_all(b"Welcome to XYZ Corp! Your mission, should you choose to accept it, is to secure our systems.")?;
    File::create(base_dir.join("documents/company_policy.txt"))?.write_all(b"1. Always be vigilant\n2. Report suspicious activities\n3. Keep your access codes secret")?;
    File::create(base_dir.join("projects/ongoing.txt"))?.write_all(b"Project Firewall: Enhance security\nProject Crypto: Implement new encryption")?;
    File::create(base_dir.join("important_note.txt"))?.write_all(b"The key to success is hidden in plain sight. Look carefully!")?;

    let mut hidden_file = File::create(base_dir.join(".security_token"))?;
    hidden_file.write_all(unique_flag.as_bytes())?;

    // Create a file that needs to be modified
    File::create(base_dir.join("projects/team_members.txt"))?.write_all(b"Alice: Project Manager\nBob: Security Analyst\nCharlie: Network Engineer")?;

    // Create a suspicious file
    File::create(base_dir.join("backups/.old_passwords.txt"))?.write_all(b"This file should not be here! Report it immediately!")?;

    println!("\nSetup complete. Your challenge environment is ready.");
    println!("Please follow these steps to start your mission:");
    println!("1. Open a new terminal window.");
    println!("2. Navigate to the mission directory: cd {}", base_dir.display());
    println!("3. Start recording your session: script mission_log.txt");
    println!("4. Complete the tasks outlined in the 'mission_briefing.txt' file.");
    println!("5. When finished, type 'exit' to stop the recording.");
    println!("6. Send both the 'mission_log.txt' file and the security token you found to HQ.");
    println!("\nGood luck, agent!");

    // Create mission briefing file
    let mut briefing_file = File::create(base_dir.join("mission_briefing.txt"))?;
    briefing_file.write_all(b"TOP SECRET - MISSION BRIEFING\n\n")?;
    briefing_file.write_all(b"Agent, your mission is to secure the XYZ Corp systems. Follow these steps:\n\n")?;
    briefing_file.write_all(b"1. Familiarize yourself with the system structure using 'ls' and 'pwd'.\n")?;
    briefing_file.write_all(b"2. Read the welcome message and company policies.\n")?;
    briefing_file.write_all(b"3. Check ongoing projects and add yourself to the team_members.txt file using 'nano'.\n")?;
    briefing_file.write_all(b"4. Create a new file named 'system_check.txt' in the main directory and write 'System checked' in it.\n")?;
    briefing_file.write_all(b"5. Search for any suspicious files or hidden information.\n")?;
    briefing_file.write_all(b"6. Locate the security token and report it to HQ.\n\n")?;
    briefing_file.write_all(b"Remember, agent, the security of XYZ Corp is in your hands. Good luck!\n")?;

    Ok(())
}