extern crate chrono;
extern crate clap;
mod client;
mod timer;

use chrono::Local;
use clap::App;
use clap::Arg;
use clap::SubCommand;
use client::Client;
use timer::LocalTimer;
use timer::Timer;
use std::thread::sleep;
use std::time::{SystemTime, Duration};
use std::collections::HashMap;


fn main() {
    println!("Tick! Tock!");
    let matches = App::new("Pocketwatch")
        .version("1.0")
        .about("Desktop Tick application.")
        .author("Jacob Pledger")
        .subcommand(SubCommand::with_name("timer")
            .arg(Arg::with_name("start")
                .takes_value(false)
                .conflicts_with("stop")
            )
            .arg(Arg::with_name("stop")
                .takes_value(false)
                .conflicts_with("start")
            )
            .arg(Arg::with_name("name")
                .takes_value(true)
                .default_value("local")
            )
        )

        .get_matches();
        let mut timers = HashMap::new();
        if let Some(matches) = matches.subcommand_matches("timer") {
            let name = matches.value_of("name").unwrap();

            // TODO: There's probably a nicer way to do this
            if timers.get(name).is_none() {
                let new_timer = LocalTimer::new();
                timers.insert(name, new_timer);
            }

            let timer = timers.get_mut(name).unwrap();

            if matches.is_present("start") {
                timer.start();
                sleep(Duration::new(5, 0));
                println!("{:?}\n", timer.get_time());
            }
            if matches.is_present("stop") {
                timer.stop();
            }
        }

}
