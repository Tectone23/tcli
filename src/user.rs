use std::{io::Write, path::PathBuf};

use crate::{
    api::create_user::RouteCreateUser,
    cipher::{decrypt_key, encrypt_key, CipherOut},
    errors::{error, info, success, throw},
    files::AppFiles,
};

pub fn add_user() {
    let file = check_user_exists();

    match file {
        Ok(file) => {
            // get username
            print!("Enter your preferred username: ");
            let _ = std::io::stdout().flush();
            let mut username = String::new();

            let reader = std::io::stdin().read_line(&mut username);
            if reader.is_err() {
                throw(&format!("{}", reader.err().unwrap()));
            };

            // get email
            let email = get_user_email();

            let username = username.replace("\n", "");
            create_user(username, email, file)
        }
        Err(err) => throw(err.as_str()),
    }
}

fn create_user(username: String, email: String, file: PathBuf) {
    let rsponse = RouteCreateUser::new();
    info("Next you will be prompted to create a password");
    info("IMPORTANT: If you lose this password, there will be no way to recover your account");

    let password = get_new_password();

    if password.is_none() {
        throw("Check if you are typing the correct password and try again");
        unreachable!();
    }

    let password = password.unwrap();

    let key = rsponse.request(username, email, String::from("User generated from tcli"));

    let cipher_out = encrypt_key(key.as_str(), password.as_str());
    success("Successfully created a user");

    let string_cipher_out = serde_json::to_string_pretty(&cipher_out);

    match string_cipher_out {
        Ok(content) => match std::fs::write(file, content) {
            Ok(_) => success("Successfully saved encrypted user data"),
            Err(err) => throw(&format!("Failed to write the keys to file: {}", err)), // TODO: Add a delete routine here
        },
        Err(err) => throw(&format!("{}", err)),
    };
}

pub fn check_user() -> Result<String, String> {
    let files = AppFiles::new();
    let mut file = files.root_dir.clone();
    file.push("user");

    if file.exists() {
        let file_text = String::from_utf8(std::fs::read(file).unwrap()).unwrap();

        let cipher_out = serde_json::from_str::<CipherOut>(file_text.as_str()).unwrap();

        let passwd = rpassword::prompt_password(format!("input your password: ")).unwrap();
        let password_input = decrypt_key(
            passwd.as_str(),
            cipher_out.txt.as_str(),
            cipher_out.nonce.as_str(),
            cipher_out.salt.as_str(),
        );

        return password_input
    } else {
        info("No existing user found");
        info("Run the following command to create a new user");
        info("tcli create-user");
        return Err(String::from("No user found"));
    }
}

fn get_new_password() -> Option<String> {
    let mut count = 0;
    loop {
        count += 1;
        let password = rpassword::prompt_password(format!("Set a strong password: ")).unwrap();
        let pass_confirm = rpassword::prompt_password(format!("Confirm your password: ")).unwrap();

        if count == 3 {
            return None;
        } else if password != pass_confirm {
            error("The two passwords don't match, please try again");
        } else {
            return Some(password);
        }
    }
}

fn check_user_exists() -> Result<PathBuf, String> {
    let files = AppFiles::new();
    let mut user_file = files.root_dir.clone();
    user_file.push("user");

    if !user_file.exists() {
        return create_user_file(user_file);
    }

    error("User already exists in $HOME/.tcli/user");
    error("DELETE the file manually to add a new user");
    error("BEWARE, this is irreversable and you will have to contact support in order to get your account back");
    error("Only one account per email is possible");
    return Err(String::from(
        "ERR: User account already exists on this computer",
    ));
}

fn create_user_file(destination: PathBuf) -> Result<PathBuf, String> {
    let file = std::fs::File::create(&destination);

    match file {
        Err(_) => return Err(String::from("Failed to create a user file")),
        Ok(_) => return Ok(destination),
    }
}

pub fn is_email(email_str: &str) -> bool {
    // REGEX - Do not tell the agency that I have used regex, mere mortals do not posess regex licenses
    let email_regex =
        regex::Regex::new(r"^([a-zA-Z0-9_\-\.+]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$").unwrap();
    return email_regex.is_match(email_str);
}

fn get_user_email() -> String {
    let mut email = String::new();
    let mut count = 0;

    loop {
        print!("Enter your email address: ");
        let _ = std::io::stdout().flush();
        let reader = std::io::stdin().read_line(&mut email);
        email = email.replace("\n", "");
        if reader.is_err() {
            throw(&format!("{}", reader.err().unwrap()));
            unreachable!();
        } else if !is_email(&email) {
            count += 1;
            error("Not a valid email addres, try again");
            if count == 3 {
                throw("Check if you have a valid email address and then try again");
                unreachable!();
            }
        } else {
            return email;
        }
    }
}
