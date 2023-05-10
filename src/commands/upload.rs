use std::fs;

use requestty::{Answers, Question};

use crate::{
    api,
    errors::{info, TcliError},
    user::check_user,
};

pub fn upload() -> Result<(), TcliError> {
    let (key, answers) = upload_cog_questions()?;

    let req = api::upload::RouteUpload::new();
    let res = req.request_from_answer(answers, key);

    match res {
        Ok(_) => return Ok(()),
        Err(err) => return Err(TcliError::RequestError(err)),
    };
}

fn upload_cog_questions() -> Result<(String, Answers), TcliError> {
    let max_len_short_desc = 50;

    if let Ok(key) = check_user() {
        info("Uploading cog");
        let questions = vec![
            Question::input("name")
                .message("What is the name of your cog")
                .build(),
            Question::input("description")
                .message("Provide a description for your cog")
                .build(),
            Question::input("short_description")
                .message(format!("Short description (max {max_len_short_desc} char)"))
                .validate(|value, _| {
                    if value.len() > max_len_short_desc {
                        return Err(String::from(
                            "Short description must be less than 255 characters",
                        ));
                    } else {
                        return Ok(());
                    }
                })
                .validate_on_key(|value, _| {
                    if value.len() > max_len_short_desc {
                        return false;
                    } else {
                        return true;
                    }
                })
                .build(),
            Question::input("version")
                .message("Version (eg. 1 : versions are single integer)")
                .validate(|value, _| {
                    if let Ok(_) = value.parse::<i32>() {
                        return Ok(());
                    } else {
                        return Err(String::from(
                            "Version must be a valid integer (eg. 1, 2, 3)",
                        ));
                    }
                })
                .validate_on_key(|value, _| {
                    if let Ok(_) = value.parse::<i32>() {
                        return true;
                    } else {
                        return false;
                    }
                })
                .build(),
            Question::input("license")
                .message("License (eg. MIT)")
                .build(),
            Question::input("issues")
                .message("Issue tracker url (eg. https://github.com/org/repo/issues)")
                .validate(|value, _| {
                    let res = reqwest::blocking::get(value);

                    match res {
                        Ok(res) => {
                            if res.status().is_success() {
                                return Ok(());
                            } else {
                                return Err(String::from("Invalid url"));
                            }
                        }
                        Err(_) => return Err(String::from("Invalid url")),
                    }
                })
                .build(),
            Question::input("app_org")
                .message("A namespace id (eg. com.example.Cog)")
                .build(),
            Question::input("file")
                .message("The packaged cog file (eg. files-cog.zip, colour.cog)")
                .validate(|value, _| {
                    if let Ok(metadata) = fs::metadata(value) {
                        if metadata.is_file()
                            && (value.ends_with(".zip") || value.ends_with(".cog"))
                        {
                            return Ok(());
                        }
                    }
                    return Err("File does not exist or is not a zip or cog file".to_string());
                })
                .validate_on_key(|value, _| {
                    if let Ok(metadata) = fs::metadata(value) {
                        return metadata.is_file()
                            && (value.ends_with(".zip") || value.ends_with(".cog"));
                    } else {
                        return false;
                    }
                })
                .build(),
        ];
        let answers = requestty::prompt(questions);

        match answers {
            Ok(answers) => return Ok((key, answers)),
            Err(err) => {
                return Err(TcliError::Other(format!(
                    "Error when processing responses: {}",
                    err
                )))
            }
        }
    } else {
        return Err(TcliError::IncorrectPassword);
    }
}
