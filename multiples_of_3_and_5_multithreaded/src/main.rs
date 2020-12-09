use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, Duration};

use multiqueue::{BroadcastReceiver, BroadcastSender};
use std::thread::sleep;

mod lib;

fn main() {
    const MAX_NUMBER: i128 = 100000;
    let number_of_work_threads = 30;
    let (counter_sender, counter_recv): (BroadcastSender<i128>, BroadcastReceiver<i128>) =
        multiqueue::broadcast_queue(10000);
    let (answer_sender, answer_revc): (BroadcastSender<i128>, BroadcastReceiver<i128>) =
        multiqueue::broadcast_queue(1000000);
    let mut handles = vec![];
    let answer = Arc::new(Mutex::new(0));

    let cur_recv = counter_recv.add_stream();
    for _thread in 0..(number_of_work_threads) {
        let stream_consumer = cur_recv.clone();
        let cur_send = answer_sender.clone();
        handles.push(thread::spawn(move || {
            for number in stream_consumer {
                let three = 3;
                let five = 5;
                if lib::multiples::is_multiple(number, three) || lib::multiples::is_multiple(number, five) {
                    cur_send.try_send(number).unwrap();
                    //println!("{}", number)
                }
            }
        }));
    }

    let answer_stream = answer_revc.add_stream().into_single().unwrap();
    {
        let answer = Arc::clone(&answer);
        handles.push(thread::spawn(move || {
            for val in answer_stream {
                //println!("val: {}", val);
                let mut temp = answer.lock().unwrap();
                *temp += val;
                //println!("temp: {}", temp);
            }
        }));
    }


    counter_recv.unsubscribe();
    answer_revc.unsubscribe();


    sleep(Duration::from_millis(500));
    let count_timer_start = SystemTime::now();
    for number in 0..MAX_NUMBER {
        let result = counter_sender.try_send(number);

        match result {
            Ok(()) => (),
            Err(error  ) => panic!("There was an error! {:?}", error),
        };
    };
    let count_timer_end = SystemTime::now();
    println!("{}, took: {}", "done loading", count_timer_end.duration_since(count_timer_start)
        .unwrap().as_millis());

    drop(counter_sender);
    drop(answer_sender);


    for handle in handles {
        // Wait for the thread to finish. Returns a result.
        println!("{}", "joining threads.....");
        handle.join().unwrap();
    }

    println!("Answer is: {}", *answer.lock().unwrap());
}


