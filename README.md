# music-player-rs

## TODOs

- [ ] clean up readme and add install instructions + demo pictures/videos (after visualizer is done)
- [ ] Cap search results based on `max_n_results` or something in `config.yaml`
- [ ] Validate user config options from `config.yaml` 
- [ ] Add `edit` command to edit song name and artist name
- [ ] Add custom `base_dir` support from `config.yaml`
- [ ] Switch to spotdl + SpotifyAPI for search 
- [ ] Display album name + album cover (and maybe lyrics) on play screen with spotdl
- [ ] Add volume normalization
- [ ] Add visualizer (RustFFT + Tokio + Rayon for async display and calculation of FFT)
- [ ] Add seeking during audio playback
- [ ] Switch to full TUI
- [X] Play audio through macos native AVFAudio + whatever windows/linux uses 
  - [https://developer.apple.com/documentation/avfaudio](https://developer.apple.com/documentation/avfaudio)
  - [https://github.com/delannoyk/AudioPlayer](https://github.com/delannoyk/AudioPlayer)
- [X] System wide key capture for forward, back and play/pause keyboard buttons (may also be os specific)
- [x] Add `move` command to move song to another playlist
- [x] Add `copy` command to copy song to another playlist (perhaps as a subcommand of `move`)
- [x] Multithread yt-dlp search for faster results
- [x] Allow download of entire yt playlist
- [x] Allow download of tracks from a given link
