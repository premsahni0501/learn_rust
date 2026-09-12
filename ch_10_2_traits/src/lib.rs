pub trait Summary {
    fn generate(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn generate(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn generate(&self) -> String {
        format!("{}: {}", self.content, self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.generate());
}
