use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();


    let v = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
        for index in 1..5 {
            println!("send x: {:?}", index);
            tx.send(index).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

   for received in rx{
       println!("Got: {:?}", received);
   }
}
