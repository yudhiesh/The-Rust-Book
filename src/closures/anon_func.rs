use std::thread;
use std::time::Duration;

// Closures have a concrete type attached to each of the variables
// They are a form of anonymous funnctions

// Using a cached closure which is stored in a struct
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // Closure is stored in Value
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                // If the closure has not been set it is set to value
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        // Here the closure is being called twice
        // Which is not optimum
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    // Currently the cacher only works when the closure passed into it
    // is of the same value
    // The following test case fails as the value of 1
    // is never replaced

    // Here any type of u32 can be used and they will all have the same value no matter what is
    // passed into the value()
    let v1 = c.value(100);
    let v2 = c.value(91919);

    assert_eq!(v1, v2);
}

#[test]
fn inline_closure() {
    let mut x: Vec<u32> = vec![1, 2, 3];
    // Even though the x is outside of the scope of the closure
    // it is still able to access it
    // as the scope of a closure is dependent on the env it is in
    // which here is the whole function
    // with int you are able to move the values without worrying about borrowing
    // as they are copied rather than moved
    // With a vec the values cannot be copied and then used again
    let equal_to_x = move |z: Vec<u32>| z == x;
    // let y: u32 = 4;
    // assert!(equal_to_x(y));
}
