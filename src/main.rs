use rand::{Rng, distributions::Alphanumeric};

fn generate_password(length: usize, character_types: Vec<char>) -> Result<String, String> {

    let mut rng = rand::thread_rng();

    let mut password = String::new();
    for _ in 0..length {
        let character_type = rng.gen::<usize>() % character_types.len();
        let character = character_types[character_type];
        password.push(character);
    }

    Ok(password)
}

fn main() {
    println!("Enter password length(8 to 16):");
    let mut length = String::new();
    std::io::stdin().read_line(&mut length).unwrap();
    let length: usize = length.trim().parse().unwrap();

    if length < 8 || length > 16 {
        panic!("Password length must be 8 to 16")
    }

    println!("Enter the desired characters :");
    let mut character_types = String::new();
    std::io::stdin().read_line(&mut character_types).unwrap();
    let character_types: Vec<char> = character_types.trim().chars().collect();

    let password = generate_password(length, character_types).unwrap();
    println!("Your password is: {}", password);
}

