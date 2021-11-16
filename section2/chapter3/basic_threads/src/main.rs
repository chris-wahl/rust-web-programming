use std::{thread, time};

use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}


async fn async_do_something(number: i8) -> i8 {
    do_something(number) // It's the same code, so just call it.
}

fn synchronous() {
    println!["\n-------SYNC---------"];
    let now = time::Instant::now();
    let one = do_something(1);
    let two = do_something(2);
    let three = do_something(3);

    println!("time elapsed: {:?}", now.elapsed());
    println!("result: {}", one + two + three);
}

fn threaded() {
    println!["\n-------THREADED---------"];
    let now = time::Instant::now();
    let thread_one = thread::spawn(|| do_something(1));
    let thread_two = thread::spawn(|| do_something(2));
    let thread_three = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let resutl_three = thread_three.join();

    println!("time elapsed: {:?}", now.elapsed());
    println!("result: {}", result_one.unwrap() + result_two.unwrap() + resutl_three.unwrap());
}

fn asynchronous() {
    println!["\n-------ASYNC---------"];
    let now = time::Instant::now();
    let future_one = async_do_something(1);
    let outcome_one = block_on(future_one);
    println!["elapsed after outcome one: {:?}", now.elapsed()];

    let future_two = async { // Can declare an async block directly
        let future_two = async_do_something(2);
        let future_three = async_do_something(3);
        join!(future_two, future_three)
    };
    let outcome_two: i8 = {
        let result = block_on(future_two);
        result.0 + result.1
    };
    println!["elapsed after outcome two: {:?}", now.elapsed()];

    let future_three = async {
        // Define a pair of futures
        let future_four = async_do_something(4);
        let future_five = async_do_something(5);
        // Store them in an array
        let futures_vec = [future_four, future_five];

        let handles = futures_vec.into_iter().map(
            async_std::task::spawn // Call the spawn function on all futures,
            // which actually concurrently runs both futures IN THE SAME THREAD, vs
            // threaded() where it starts a new thread each time
        ).collect::<Vec<_>>(); // and collect the results into a vector
        let results = join_all(handles).await;
        return results;
    };
    let outcome_three: i8 = block_on(future_three).into_iter()
        .reduce(|p, n| p + n).unwrap();

    println!("total time elapsed: {:?}", now.elapsed());
    println!("Outcome: {}", outcome_one + outcome_two + outcome_three);
}


fn main() {
    synchronous();
    threaded();
    asynchronous();
}
