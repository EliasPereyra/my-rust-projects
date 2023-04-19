mod datatypes;
mod error_handling;
mod generic_types;

use crate::garden::vegetables::Asparagus;

pub mod garden;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}

fn main() {
    // print::run();
    // datatypes::vars::run();
    // types::run();
    // datatypes::strings::run();
    // vectors::run();
    // tuples::run();
    // conditionals::run();
    // arrays::run();
    // functions::run();
    // structs::run();
    // enums::run();
    // datatypes::hashmaps::run();
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    // error_handling::unrecoverable_errors::run();
    // error_handling::recoverable_errors::run();
    // error_handling::recoverable_errors::read_email_from_file();
    // generic_types::remove_duplications::run();

    let tweet = Tweet {
        username: String::from("tech_news"),
        content: String::from("Big companies are hacked very often"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
