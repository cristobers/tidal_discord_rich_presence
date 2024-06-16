# Tidal Rich Presence for Discord

Allows TIDAL users to share what they are listening to with Discord.

## Setup

1. `cargo build --release` to build `tidal-richpresence.exe`
2. Join https://discord.com/oauth2/authorize?client_id=1242553912322560122
3. Copy the `tidal_richpresence_config.json` file and place it somewhere accessible.
4. Run `tidal_richpresence.exe` followed by the path for the config.
5. If TIDAL is currently playing music, pause it and wait for ~4 seconds, TIDAL should 
then be found by the executable and should start sending rich presence information to 
Discord.
