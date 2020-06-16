use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use neu;

struct Cacher<T>
where
    T: Fn(usize) -> usize,
{
    calculation: T,
    value: HashMap<usize, usize>,
}

impl<T> Cacher<T>
where
    T: Fn(usize) -> usize,
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calculation: calc,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: usize) -> usize {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: usize, random_number: usize) {
    let mut expensive_res = Cacher::new(|num| {
        println!("calc slow");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_res.value(intensity));
        println!("Today, do {} situps", expensive_res.value(intensity));
    } else {
        if random_number == 3 {
            println!("lucky you, take a break, drink some water");
        } else {
            println!("Today, run for {} minutes!", expensive_res.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    //    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;

    let equal_to_x = move |z| z == x;
    // can't use x after
    let y = 4;
    assert!(equal_to_x(y));
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_gen() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter().map(|x| x + 1);
    let v2: Vec<i32> = v1_iter.collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
