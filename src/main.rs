#![windows_subsystem = "windows"]
mod tidal;
mod discord;

use std::fs;
use discord_sdk as ds;
use windows_sys::Win32::Foundation::HWND;
use std::{thread, time, env};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args()
        .collect::<Vec<String>>();
    let config: Config = read_from_file(&args[1]);
    let playing_img: &str = &config.playing_img;
    let discord_delay: u64 = config.discord_delay;

    let mut large_img; 
    let mut last_song = String::from("Empty");

    let mut parsed: tidal::Title = tidal::Title {
        song: "None".to_string(),
        artist: "None".to_string(),
    };

    let client = discord::make_client(ds::Subscriptions::ACTIVITY).await;
    let mut tidal_hwnd = try_for_hwnd();
    loop {
        let curr_song = tidal::check_title(tidal_hwnd);
        match curr_song {
            // If we're successful in checking the title for TIDAL.
            Ok(mut curr_song) => {
                if curr_song != last_song {
                    last_song = curr_song.clone();
                    curr_song = curr_song.split('\0').collect();

                    if we_are_paused(&curr_song) {
                        if we_have_previously_played_a_song(&parsed.song) {
                            let _ = client.discord.clear_activity().await;
                        } else {
                            // If we are paused, 
                            // and we haven't previously played a song,
                            // do nothing, wait a little bit, and try again.
                            thread::sleep(time::Duration::from_secs(3));
                            continue;
                        }
                    } else {
                        // If we aren't paused, parse the title from TIDAL and update 
                        // Discord.
                        parsed = parse_song(curr_song);
                        large_img = &config.large_playing_img;
                        let _ = client.discord.update_activity(
                            ds::activity::ActivityBuilder::default()
                                .details(parsed.song.to_owned())
                                .state(parsed.artist.to_owned())
                                .assets(
                                    activity_assets(
                                        large_img,
                                        "oooh yeah music time",
                                        playing_img,
                                        "Playing"
                                    )
                                )
                        ).await;
                    }
                }
                thread::sleep(time::Duration::from_secs(discord_delay));
            }
            // Otherwise TIDAL may have been previously closed, try and refind TIDAL.
            Err(_) => {
                tidal_hwnd = try_for_hwnd();
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    playing_img: String,
    paused_img : String,
    large_playing_img: String,
    large_paused_img : String,
    discord_delay: u64
}

fn read_from_file(filepath: &str) -> Config {
    let file = fs::read_to_string(filepath)
        .expect("Unable to open file at {filepath}");
    
    let config: Config = serde_json::from_str(&file)
        .expect("Failed to parse config.");
    config
}

fn try_for_hwnd() -> HWND {
    loop {
        match tidal::get_tidal_hwnd() {
            Ok(value) => {
                println!("tidal found {:?}", value);
                return value;
            }
            Err(_) => {
                println!("retrying...");
                thread::sleep(time::Duration::from_secs(3));
            }
        }
    }
}

fn parse_song(curr_song: String) -> tidal::Title {
    // Takes in the title from TIDAL and creates a `tidal::Title` struct.
    let split_title: Vec<&str> = curr_song.split(" - ").collect();
    tidal::Title {
        song:   split_title[0].to_string(),
        artist: split_title[1].to_string(),
    }
}

fn we_are_paused(curr_song: &str) -> bool {
    curr_song == "TIDAL"
}

fn we_have_previously_played_a_song(prev_song: &str) -> bool {
    prev_song != "TIDAL" && prev_song != "None"
}

fn activity_assets(big_img: &str, b_txt: &str, small_img: &str, s_txt: &str) 
    -> ds::activity::Assets {
    ds::activity::Assets::default()
        .large(
            big_img.to_owned(),
            Some(b_txt).to_owned()
        )
        .small(
            small_img.to_owned(),
            Some(s_txt).to_owned()
        )
}
