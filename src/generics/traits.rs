// Traits are a way for a particular type to be shared amongst other types
// Similar to other languages which have a interfaces
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub total_views: u32,
}

#[derive(Debug)]
pub struct Tweets {
    pub name: String,
    pub date: String,
    pub characters_length: i32,
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn greet(&self) -> String;
}

impl Summary for Tweets {
    fn summarize(&self) -> String {
        format!(
            "Tweet by {} on the {} which has a {} characters",
            self.name, self.date, self.characters_length
        )
    }
    fn summarize_author(&self) -> String {
        format!("Tweet was posted by {} and on the {}", self.name, self.date)
    }
    fn greet(&self) -> String {
        format!("Hello {}", self.name)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
    fn summarize_author(&self) -> String {
        format!(
            "{} is located in {} and has gotten {} views for this article",
            self.author, self.location, self.total_views
        )
    }
    fn greet(&self) -> String {
        format!("Hello {}", self.author)
    }
}

// Display multiple trait bounds by using
// pub fn notify(item: &(impl Summary + Display))

pub fn notify(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news!");
    println!("Article from {}", item.summarize());
    println!("Tweet from {}", item2.summarize_author());
}

// Can only return a single impl not multiple
pub fn returns_summarizable() -> impl Summary {
    Tweets {
        name: String::from("John"),
        date: String::from("1st of December"),
        characters_length: 102,
    }
}

pub fn random_data(item: &impl Summary) -> String {
    format!("Tweet from {}", item.summarize_author())
}

// With a non generic version we made sure that the type passed into the function
// has values that can be passed into and compared to
// this cannot be done for items such as Strings

// Rust has a special annotation called Copy that can be placed on ints that are stored on the
// stack
// If a type has a copy trait, an older variable is still usable on the stack after assignment
// Types that are copy:

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that are also Copy.
// For example, (i32, i32) is Copy, but (i32, String) is not.
// When making the generic largest function it is possible
// to have items that do not implement the copy trait
// So we need to specify it

// PartialOrd enables comparison

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Using trait bounds to conditionally implement methods

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println! {"{} is greater than {}", self.x, self.y}
        } else {
            println! {"{} is greater than {}", self.y, self.x}
        }
    }
}

// Code for the main function

//let article1 = traits::NewsArticle {
//    headline: String::from("Wow that is great!"),
//    location: String::from("Kuala Lumpur"),
//    author: String::from("Yudhiesh"),
//    total_views: 100,
//};
//let tweet1 = traits::Tweets {
//    name: String::from("Michelle"),
//    date: String::from("1st of October"),
//    characters_length: 32,
//};
//// println!("New article is available: {}", article1.summarize());
//// println!("Details about the article {}", article1.summarize_author());
//// traits::notify(&article1, &tweet1);
//let tweet2 = traits::returns_summarizable();
////println!("{}", tweet2.greet());
//let r = traits::random_data(&tweet2);
////println!("{}", r);
//
//let number_list = vec![34, 50, 25, 100, 65];
//
//let result = traits::largest(&number_list);
//println!("The largest number is {}", result);
//
//let char_list = vec!['y', 'm', 'a', 'q'];
//
//let result = traits::largest(&char_list);
//println!("The largest char is {}", result);
