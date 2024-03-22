#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use check_if_email_exists::{
    check_email, mx, smtp, syntax, CheckEmailInput, CheckEmailInputProxy, CheckEmailOutput,
};
use rand::*;
use rocket::futures::executor::block_on;
use rocket::futures::future::{AbortHandle, Abortable, Aborted};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub async fn bulkcheckemails(
    from: String,
    listemails: Vec<String>,
    smpttimeout: Option<u64>,
) -> Vec<CheckEmailOutput> {
    let mut results: Vec<CheckEmailOutput> = Vec::with_capacity(listemails.len());
    let mut children = Vec::new();
    // let myproxy = (
    //     "socks5.kmoz.dev",
    //     8318,
    //     "identityserver4",
    //     "4b2606a107223dd0d",
    // );
    let smtptimeout_received = match smpttimeout.clone() {
        Some(m) => {
            if m > 500 {
                Duration::from_millis(m as u64)
            } else {
                Duration::from_millis(900)
            }
        }
        None => Duration::from_millis(900),
    };
    let (tx, rx): (Sender<CheckEmailOutput>, Receiver<CheckEmailOutput>) = mpsc::channel();
    for elem in listemails.clone() {
        let mut input = CheckEmailInput::new(elem.clone().into());
        let hello = elem.split("@").into_iter().nth(1).unwrap();
        input
            .set_from_email(from.clone()) // Used in the `MAIL FROM:` command
            .set_hello_name(hello.into());
        if smtptimeout_received.as_millis() >= 300 {
            {
                input.set_smtp_timeout(smtptimeout_received);
            }
            let thread_tx = tx.clone();
            let child = thread::spawn(move || {
                block_on(async {
                    let found = check_email(&input).await;
                    thread_tx.send(found).unwrap();
                })
            });
            children.push(child);
        }
    }
    // Here, all the messages are collected
    for _ in 0..listemails.clone().len() {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        results.push(rx.recv().expect("oops! the recv() panicked"));
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    return results;
}
