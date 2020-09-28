#[derive(Debug)]
struct House {
    land_size: u32,
    value: u32,
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

impl House {
    fn can_hold(&self, item: &House) -> bool {
        self.land_size > item.land_size && self.value > item.value
    }
    fn equals(&self, item: &House) -> bool {
        self.land_size == item.land_size && self.value == item.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn houses() {
        let house1 = House {
            land_size: 200_000,
            value: 1_000_000,
        };
        let house2 = House {
            land_size: 300_000,
            value: 2_000_000,
        };
        // Custom error message
        assert!(house2.can_hold(&house1), "Error at `{:#?}`", house2)
    }
    #[test]
    fn is_equal() {
        let house1 = House {
            land_size: 200_000,
            value: 1_000_000,
        };
        let house2 = House {
            land_size: 300_000,
            value: 2_000_000,
        };
        assert_eq!(house1.equals(&house1), true);
        assert_eq!(house2.equals(&house2), true);
        assert_ne!(house1.equals(&house1), false);
        assert_ne!(house1.equals(&house1), false);
    }
    //#[test]
    //#[should_panic(expected = "Guess value must be less than or equal to 100")]

    //fn greater_than_100() {
    //    Guess::new(10);
    //}

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from(
                "Error occured as one plus two does not equal to four",
            ))
        }
    }
}
