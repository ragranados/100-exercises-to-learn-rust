// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let new_v = Box::new(v);
    let leaked: &mut Vec<i32> = Box::leak(new_v);

    let first_thread: thread::JoinHandle<i32> =
        thread::spawn(|| leaked.as_slice()[0..leaked.len() / 2].iter().sum());

    let second_thread: thread::JoinHandle<i32> = thread::spawn(|| {
        leaked.as_slice()[leaked.len() / 2..leaked.len()]
            .iter()
            .sum()
    });

    first_thread.join().unwrap() + second_thread.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
