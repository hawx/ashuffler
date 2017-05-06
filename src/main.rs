extern crate clap;
extern crate mpd;
extern crate rand;

use clap::{Arg, App};
use mpd::{Client, Idle};
use mpd::song::Song;
use mpd::idle::Subsystem;
use rand::{Rng, StdRng};

fn main() {
    let matches = App::new("ashuffler")
        .arg(Arg::with_name("only")
                 .long("only")
                 .value_name("NUM")
                 .takes_value(true)
                 .help("Instead of continuously adding songs, just add 'NUM' songs and then exit"))
        .arg(Arg::with_name("buffer")
                 .long("buffer")
                 .value_name("NUM")
                 .takes_value(true)
                 .help("Always keep 'NUM' songs in the queue after the one currently playing"))
        .arg(Arg::with_name("mpdUrl")
                 .long("mpd-url")
                 .value_name("URL")
                 .takes_value(true)
                 .required(false)
                 .help("URL to mpd instance (default: 127.0.0.1:6600)"))
        .get_matches();

    let mpd_url = matches.value_of("mpdUrl").unwrap_or("127.0.0.1:6600");
    let mut conn = Client::connect(mpd_url).expect("Could not connect to mpd");
    let mut songs = conn.listall().expect("Could not get song list");

    let mut rng = StdRng::new().expect("Could not shuffle song list");
    rng.shuffle(&mut songs);

    if songs.len() == 0 {
        println!("Song pool is empty");
        return;
    }
    println!("Picking random songs out of a pool of {}", songs.len());

    let only = matches
        .value_of("only")
        .and_then(|x| x.parse::<i32>().ok());

    match only {
        Some(only) => {
            if only < 1 {
                println!("Can only queue a positive number of songs");
                return;
            }

            println!("Adding {} songs", only);
            shuffle_only(&mut conn, &mut songs, only);
        }
        None => {
            let buffer = matches
                .value_of("buffer")
                .and_then(|x| x.parse::<i32>().ok())
                .unwrap_or(1);

            if buffer < 1 {
                println!("Can only keep a buffer of a positive number of songs");
                return;
            }

            println!("Continuously maintaining a buffer of {} songs", buffer);
            shuffle_idle(&mut conn, &mut songs, buffer);
        }
    }
}

fn shuffle_only(conn: &mut Client, songs: &mut Vec<Song>, only: i32) {
    for _ in 0..only {
        songs
            .pop()
            .ok_or("Could not get song from list")
            .and_then(|song| {
                          conn.push(song)
                              .map_err(|_| "Could not add song to queue")
                      })
            .unwrap();
    }
}

fn shuffle_idle(conn: &mut Client, songs: &mut Vec<Song>, buffer: i32) {
    loop {
        let status = conn.status().expect("Could not get mpd status");

        let queue_length = status.queue_len as i32;
        let current_pos = status.song.map(|x| x.pos as i32).unwrap_or(0);
        let current_buffer = queue_length - current_pos - 1;
        let diff = buffer - current_buffer;

        if diff > 0 {
            shuffle_only(conn, songs, diff);
        }

        conn.wait(&[Subsystem::Player])
            .expect("Failed to wait on mpd");
    }
}
