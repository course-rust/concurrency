use std::{sync::mpsc, thread, time};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

/// 命令行运行
///
/// ```cargo run --color=always --package template --example thread1```
///
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // 释放 tx， 否则 rx 无法结束

    // 创建 consumer
    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    let sec = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("Consumer sec: {}", sec);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(time::Duration::from_millis(sleep_time));

        // random exit the producer
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
