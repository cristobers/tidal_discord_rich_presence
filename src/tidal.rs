use reqwest::Client;
use windows_sys::{
    core::*, Win32::{Foundation::HWND, UI::WindowsAndMessaging::*}
};

#[derive(Debug, Clone)]
pub struct Title {
    pub song: String,
    pub artist: String,
}

async fn make_tidal_api_req(url: &str) -> serde_json::Value {
    let client = Client::new();
    let resp = client
        .get(url)
        .header("x-tidal-token", "zU4XHVVkc2tDPo4t")
        .send()
        .await.unwrap();
    dbg!(&resp);
    resp.json().await.unwrap()
}

fn make_tidal_api_url(curr_song: Title) -> String { 
    let parsed_song   = curr_song.song.replace(" ", "%20");
    let parsed_artist = curr_song.artist.replace(" ", "%20");
    let mut final_parsed = String::from(parsed_song);
    final_parsed.push('-');
    final_parsed.push_str(&parsed_artist);
    // TODO: try and determine the country code of the user for better results
    format!(
        "https://api.tidal.com/v1/search?query={}&limit=50&offset=0&types=TRACKS&countryCode=US", 
        final_parsed
    )
}

pub async fn get_song_image(curr_song: Title, if_fails: &str) -> String {
    // check notes to see how to make this part work
    // should probably cache responses so we make less web requests
    // that way tidal hates me less :D
    let url = make_tidal_api_url(curr_song);
    let resp = make_tidal_api_req(&url).await;

    // this sometimes gives the wrong album cover.
    let cover = &mut resp["tracks"]["items"][0]["album"]["cover"].as_str();
    match cover {
        Some(val) => {
            let mut base_url = String::from("https://resources.tidal.com/images/");
            // TODO: URL ENCODE THIS INSTEAD OF REPLACING VALUES
            // NOT URL ENCODING CAUSES CERTAIN SONGS TO FAIL "prodigy break & enter"
            let mut replaced_cover = val.replace("-", "/");
            replaced_cover.push_str("/80x80.jpg");
            base_url.push_str(&replaced_cover);
            base_url
        }
        None => {
            dbg!("COVER IS NONE!!!");
            if_fails.to_owned()
        }
    }
}

pub fn get_tidal_hwnd() -> Result<HWND, String> {
    // Tries to get the `HWND` for TIDAL.
    unsafe {
        let res: HWND = FindWindowA(std::ptr::null(), s!("TIDAL"));
        match res {
            0 => Err("couldn't find tidal".to_string()),
            _ => Ok(res)
        }
    }
}

pub fn check_title(w: HWND) -> Result<String, String> {
    // Tries to find the window title "TIDAL" for a given HWND.
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let res = GetWindowTextW(w, text.as_mut_ptr(), text.len() as i32);
        match res {
            0 => Err("TIDALERROR: failed to get text.".to_string()),
            _ => Ok(String::from_utf16(&text).unwrap())
        }
    }
}
