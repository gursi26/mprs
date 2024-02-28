# music-player-rs

## TODOs

- [ ] Move user track selection to separate function + add error checking for user input
- [ ] Add `edit` command to edit song name and artist name
- [ ] Add visualizer (RustFFT + Tokio + Rayon for async display and calculation of FFT)
- [ ] Switch to spotdl + SpotifyAPI for search (client credentials oauth flow for SpotifyAPI)
- [ ] Display album name + album cover (and maybe lyrics) on play screen with spotdl
- [ ] Switch to full TUI
- [ ] Play audio through macos native AVFAudio + whatever windows/linux uses 
    - [https://developer.apple.com/documentation/avfaudio](https://developer.apple.com/documentation/avfaudio)
    - [https://github.com/delannoyk/AudioPlayer](https://github.com/delannoyk/AudioPlayer)
- [ ] System wide key capture for forward, back and play/pause keyboard buttons (may also be os specific)
- [x] Add `move` command to move song to another playlist
- [x] Add `copy` command to copy song to another playlist (perhaps as a subcommand of `move`)
- [x] Multithread yt-dlp search for faster results
- [x] Allow download of entire yt playlist
- [x] Allow download of tracks from a given link
