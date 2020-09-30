struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(vec: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    vec.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // iter() produces an Iterator over immutable references
    // into_iter() takes ownership of the values and then returns owned values
    // iter_mut() iterates over mutable references

    // methods that call next() are called consuming adaptors as calling them uses up the iterator
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum() takes ownership of the iterator
    // so you are not able to use v1_iter() as it belongs to sum()
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn lazy_iterator() {
    // Iterator adopters allow you to chain multiple calls to iterator adopters
    // to perform complex actions in a readable way
    // But as iterators are lazy you need to consume the iterator after it
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn find_show_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 12,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let my_show_size = shoes_in_my_size(shoes, 12);
    assert_eq!(
        my_show_size,
        vec![Shoe {
            size: 12,
            style: String::from("sandal")
        }]
    )
}
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn combine_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(x, y)| x * y)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
