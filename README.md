# Tidal Rich Presence for Discord

Allows TIDAL users to share what they are listening to with Discord.

![Discord_zFkzDNJInY](https://github.com/user-attachments/assets/eda98959-98c6-4c0d-a961-ac72131584e7)

Note: this is mostly just for me and may (and probably will) break and not be fixed for long periods of time. If you're going to use this, make sure you're comfortable with Rust.

## Setup

1. `cargo build --release` to build `tidal-richpresence.exe`
2. Join https://discord.com/oauth2/authorize?client_id=1242553912322560122 (idk why it says it can send direct messages i never want to send direct messages.)
3. Copy the `tidal_richpresence_config.json` file and place it somewhere accessible.
4. Run `tidal_richpresence.exe` followed by the path for the config.
5. If TIDAL is currently playing music, pause it and wait for ~4 seconds, TIDAL should 
then be found by the executable and should start sending rich presence information to 
Discord.

### Running on startup

1. Do steps 1-3 from the section above
2. Create a shortcut for `tidal_richpresence.exe`, place this within the Windows 
Start-up folder
3. Edit the shortcuts target (Right click, Properties) to look something like this:
<PATH WHERE EXE LIVES> <PATH TO tidal_richpresence.exe>
4. It's probably good to also have Discord auto launch
