#![windows_subsystem = "windows"]
mod tidal;
mod discord;
use discord_sdk as ds;
use windows_sys::Win32::Foundation::HWND;
use std::{thread, time};

fn activity_assets(big_img: &str, b_txt: &str, small_img: &str, s_txt: &str) -> ds::activity::Assets {
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

#[tokio::main]
async fn main() {
    let mut large_img; 
    const PLAYING_IMG: &str = "https://cdn.discordapp.com/app-assets/1242553912322560122/1243945778225877043.png";
    const PAUSED_IMG: &str = "https://cdn.discordapp.com/app-assets/1242553912322560122/1243947849834823711.png";

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
                            large_img = "https://media1.tenor.com/m/t4j2MWEZgSEAAAAd/kitten.gif";
                            // Take the previously played song, give it the small paused img.
                            let _ = client.discord.update_activity(
                                ds::activity::ActivityBuilder::default()
                                    .details(parsed.song.to_owned())
                                    .state(parsed.artist.to_owned())
                                    .assets(
                                        activity_assets(
                                            large_img,
                                            "sleeping bc no music",
                                            PAUSED_IMG,
                                            "Paused"
                                        )
                                    )
                            ).await;
                        } else {
                            // If we are paused, and we haven't previously played a song,
                            // do nothing, wait a little bit, and try again.
                            thread::sleep(time::Duration::from_secs(3));
                            continue;
                        }
                    } else {
                        // If we aren't paused, parse the title from TIDAL and update Discord.
                        parsed = parse_song(curr_song);
                        large_img = "https://media.tenor.com/KC4-Zja4V7gAAAAi/cat-jam.gif";
                        let _ = client.discord.update_activity(
                            ds::activity::ActivityBuilder::default()
                                .details(parsed.song.to_owned())
                                .state(parsed.artist.to_owned())
                                .assets(
                                    activity_assets(
                                        large_img,
                                        "oooh yeah music time",
                                        PLAYING_IMG,
                                        "Playing"
                                    )
                                )
                        ).await;
                    }
                }
                thread::sleep(time::Duration::from_secs(4));
            }
            // Otherwise TIDAL may have been previously closed, try and refind TIDAL.
            Err(_) => {
                tidal_hwnd = try_for_hwnd();     
            }
        }

    }
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
