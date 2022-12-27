
mod aggregator;


use aggregator::{Summary, Tweet, NewsArticle};

pub fn dosth() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    print_summary(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    article.summarize();

    print_summary(&article);
}

fn print_summary(item: &impl Summary) {
    println!("1 new tweet: {}", item.summarize());
}

//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function_2<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}
