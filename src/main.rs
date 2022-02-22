use rust_generic_types_traits_lifetimes::aggregator::{NewsArticle, Pair, Summary, Tweet};

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let float_list = vec![89.36, 67.2, 34.8, 12.7];
    let result = largest(&float_list);
    println!("The largest float is {}", result);

    let string_list = vec!["abc", "zdef", "zzz"];
    let result = largest(&string_list);
    println!("The largest string is {}", result);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("some content"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("john doe"),
        content: String::from("content"),
    };
    println!("New article available! {}", article.summarize());

    let pair = Pair::new(9, 8);
    pair.cmp_display();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
