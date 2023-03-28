use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

use crate::errors::throw;

pub struct RouteCreateUser {
    api: String,
    route: String,
}

#[derive(Serialize)]
struct ReqBody {
    name: String,
    email: String,
    bio: String,
}

impl RouteCreateUser {
    pub fn new() -> Self {
        return RouteCreateUser {
            api: String::from("20.86.26.32:8000"),
            route: String::from("register_user"),
        };
    }

    pub fn request(&self, name: String, email: String, bio: String) -> String {
        let cl = reqwest::blocking::Client::new();
        let sendth = ReqBody { name, email, bio };

        let url = format!("http://{}/{}", self.api, self.route);
        let req = cl.request(reqwest::Method::POST, url);
        let req = req.json(&sendth);

        let res = req.send();

        match res {
            Ok(res) => {
                let status = res.status();
                if status != 200 {
                    throw(&format!("Request returned with status code: {status}"));
                    unreachable!()
                }

                let js = self.get_json(res);
                return js.token;
            }
            Err(err) => {
                throw(&format!("Request to server failed with error: {err}"));
                unreachable!()
            }
        }
    }

    fn get_json(&self, res: Response) -> Res {
        match res.json::<Res>() {
            Ok(json) => return json,
            Err(err) => {
                throw(&format!("Parsing response failed with error: {err}"));
                unreachable!()
            }
        };
    }
}

#[derive(Serialize, Deserialize)]
struct Res {
    token: String,
}
