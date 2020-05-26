extern crate mongodb;
extern crate rand;

use mongodb::{bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::thread;

fn main() {

    let handle = thread::spawn(move || {
        let client = Client::connect("localhost", 27111).expect("Failed to initialize standalone client.");

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(300)
            .collect();

        let doc = doc! {
                "title": rand_string,
                "array": [ 1, 2, 3 ],
            };

        for _x in 0..10000 {
            client.db("test").collection("movies").insert_one(doc.clone(), None)
                .ok().expect("Failed to insert document.");
        }
    });

    handle.join().unwrap();
}