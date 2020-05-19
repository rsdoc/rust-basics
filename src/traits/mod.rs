//! trait
//!  - interface
//!  - compulsory to define all the methodsof the trait
trait Summary {
    // fn summarize(&self) -> String;
    // we can have default implementatio for the above methods
    fn summarize(&self) -> String {
        format!(" Read more .....")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// implements trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})  -> {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

// creating other struct to implements the same summarize trait
struct Tweet {
    username: String,
    content: String,
    reply: String,
    retweet: String,
}

// if we do not impl the trait - default implementation will be called
impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!(
    //         " {} by {}, click here to reply {} and retweet {}",
    //         self.content, self.username, self.reply, self.retweet
    //     )
    // }
}

pub fn run() {
    // creating instance of struct
    let news = NewsArticle {
        headline: String::from("Learn rust by doing ✍"),
        location: String::from(" India "),
        author: String::from(" Rahul Singh"),
        content: String::from(" Install Rust - rust-lang.org "),
    };

    println!(" the blog  is {:?}", news.summarize());

    let tweet = Tweet {
        username: String::from(" Raghu"),
        content: String::from(" Good Article "),
        reply: String::from(" add some features"),
        retweet: String::from(" do retweet "),
    };

    println!(" the tweet - {:?}", tweet.summarize());
}
