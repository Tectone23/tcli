use std::path::PathBuf;

use requestty::{Answer, Answers};
use reqwest::blocking::multipart::{Form, Part};

pub struct RouteUpload {
    api: String,
    route: String,
}

impl RouteUpload {
    pub fn new() -> Self {
        return RouteUpload {
            api: String::from("20.86.26.32:8000"),
            route: String::from("manage/upload_cog"),
        };
    }

    pub fn request(
        &self,
        key: String,
        name: String,
        description: String,
        short_description: String,
        version: String,
        license: String,
        issues: String,
        app_org: String,
        file: PathBuf,
    ) -> Result<(), String> {
        let cl = reqwest::blocking::Client::new();

        let form = Form::new()
            .text("key", key)
            .text("name", name)
            .text("description", description)
            .text("short_description", short_description)
            .text("version", version)
            .text("license", license)
            .text("issues", issues)
            .text("cog_org", app_org)
            .part("file", Part::file(file).unwrap());

        let url = format!("http://{}/{}", self.api, self.route);
        let req = cl.request(reqwest::Method::POST, url);
        let req = req.multipart(form);

        let res = req.send().unwrap();


        if res.status() != 200 {
            return Err(format!(
                "Failed to send request with status code: {}",
                res.text().unwrap()
            ));
        } else {
            return Ok(());
        }
    }

    pub fn request_from_answer(&self, answers: Answers, key: String) -> Result<(), String> {
        let name = self.get_string_from_answer(&answers.get("name").unwrap());
        let description = self.get_string_from_answer(&answers.get("description").unwrap());
        let short_description =
            self.get_string_from_answer(&answers.get("short_description").unwrap());
        let version = self.get_string_from_answer(&answers.get("version").unwrap());
        let license = self.get_string_from_answer(&answers.get("license").unwrap());
        let issues = self.get_string_from_answer(&answers.get("issues").unwrap());
        let app_org = self.get_string_from_answer(&answers.get("app_org").unwrap());
        let file = self.get_string_from_answer(&answers.get("file").unwrap());
        let file = PathBuf::from(file);

        return self.request(
            key,
            name,
            description,
            short_description,
            version,
            license,
            issues,
            app_org,
            file,
        );
    }

    fn get_string_from_answer(&self, answer: &Answer) -> String {
        match answer {
            Answer::String(s) => return s.clone(),
            _ => return String::new(),
        }
    }
}
