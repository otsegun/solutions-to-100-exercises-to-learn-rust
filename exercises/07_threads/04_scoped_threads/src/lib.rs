// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::{Scope, ScopedJoinHandle};
pub fn sum2(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    std::thread::scope(|s| {
        let left = s.spawn(|| left.iter().sum::<i32>());
        let right = s.spawn(|| right.iter().sum::<i32>());
        left.join().unwrap() + right.join().unwrap()
    })
}

pub fn sum(v: Vec<i32>) -> i32 {
    let length: usize = v.len();
    let midpoint: usize = length /2;


    std::thread::scope(|scope|{
        let first_handle: ScopedJoinHandle<'_, i32> = scope.spawn(||{
            let first = &v[..midpoint]; 
            first.iter().sum()
            
        });

        let second_handle: ScopedJoinHandle<'_, i32> = scope.spawn(||{
            let second = &v[midpoint..];
           second.iter().sum()
        });      

        first_handle.join().unwrap() + second_handle.join().unwrap()
    })
    
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
