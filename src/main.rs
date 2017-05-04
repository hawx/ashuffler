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
    let mut conn = Client::connect(mpd_url).unwrap();
    let mut rng = StdRng::new().unwrap();

    let mut songs = conn.listall().unwrap();
    rng.shuffle(&mut songs);

    if songs.len() == 0 {
        println!("Song pool is empty");
        return;
    }
    println!("Picking random songs out of a pool of {}", songs.len());

    match matches.value_of("only").and_then(|x| {
        x.parse::<i32>().ok()
    }) {
        Some(only) => {
            if only < 1 {
                println!("Can only queue a positive number of songs");
                return
            }

            println!("Adding {} songs", only);
            shuffle_only(&mut conn, &mut songs, only);
        },
        None => {
            let buffer = matches.value_of("buffer").and_then(|x| {
                x.parse::<i32>().ok()
            }).unwrap_or(1);

            if buffer < 1 {
                println!("Can only keep a buffer of a positive number of songs");
                return
            }

            println!("Continuously maintaining a buffer of {} songs", buffer);
            shuffle_idle(&mut conn, &mut songs, buffer);
        }
    }
}

fn shuffle_only(conn: &mut Client, songs: &mut Vec<Song>, only: i32) {
    for _ in 0..only {
        conn.push(songs.pop().unwrap()).unwrap();
    }
}

fn shuffle_idle(conn: &mut Client, songs: &mut Vec<Song>, buffer: i32) {
    loop {
        conn.wait(&[Subsystem::Player]).unwrap();

        let status = conn.status().unwrap();

        let queue_length = status.queue_len as i32;
        let current_pos = status.song.map(|x| x.pos as i32).unwrap_or(0);
        let current_buffer = queue_length - current_pos - 1;
        let diff = buffer - current_buffer;

        if diff > 0 {
            for _ in 0..diff {
                conn.push(songs.pop().unwrap()).unwrap();
            }
        }
    }
}
