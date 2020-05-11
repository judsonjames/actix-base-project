use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    title: String,
    body: String
}

impl BlogPost {
    pub fn new() -> BlogPost {
        BlogPost{
            title: "Test Blog Post Title".to_string(),
            body: "Test Blog Post Body".to_string(),
        }
    }
}