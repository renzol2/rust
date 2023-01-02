use std::sync::mpsc::*;
use std::thread;
use std::thread::JoinHandle;

/// TODO: Implement compute 
/// Take vector of integers as parameters
/// Return a new vector containing all the integers that 
/// have the sum of their digits divisible by 9
pub fn compute(keys: Vec<i32>) ->  Vec<i32> {
    todo!();
}

/// TODO: Implement split
/// Take in a vector of integers and an integer
/// Split the Vector of integer into smaller vectors of size num_per_chunk
/// Create a thread in which you call compute on the smaller vector
/// Return a tuple that contains a vector of the join handles and the receiver
/// WE WILL GIVE NICE NUMBERS
pub fn split(num_per_chunk: i32, keys: Vec<i32>) -> (Vec<JoinHandle<()>>, Receiver<Vec<i32>>) {
    todo!();
}

/// TODO: Implement Join
/// Take in a vector of join handles and the receiver from the previous function
/// Have the receiver receive the values from each transmitter and
/// put together the received value to create an output Vec<i32> of all the integers in the original list
/// whose digits sum to a number divisible by 9
pub fn join(a : Vec<JoinHandle<()>>, b : Receiver<Vec<i32>>) -> Vec<i32> {
    todo!();
}

// [HELPER FUNCTION]
pub fn runner(chunk_size: i32, numbers: Vec<i32>)-> Vec<i32> {
    let (x,y) = split(chunk_size, numbers);
    let output = join(x,y);
    return output;
}