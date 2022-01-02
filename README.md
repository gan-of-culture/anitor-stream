# anitor-stream

Search and live stream anime/hentai torrent from the cli.

## Installation

For this script to work you need to have this dependency installed:
```bash
webtorrent-cli
```

Download the latest binary from the [releases page](https://github.com/gan-of-culture/anitor-stream/releases).

Then move into the directory and make the binary executable:
```bash
chmod +x anitor-stream
```

## Usage

```bash
./anitor-stream [SEARCH_QUERY]
```
Replace the ```SEARCH_QUERY``` with the name of the show and episode you wanna watch. For example:

```bash
./anitor-stream "Mahouka Koukou no Yuutousei"
```

After that you can enter the torrent category. For Eng-Subs choose the first opiton. Now up too 74 available torrents will be listed.
Enter the number of the torrent you want to watch and press enter. Now the video should start playing.

If you don't enter a search query you'll be presented with all shows that aired in the last 24 hours. Again choose the search term by it's number. If you want to choose a different player than mpv you can do so by suppling one of these terms with -p or --player.

```bash
airplay
chromecast
mplayer
mpv (default)
vlc
xbmc
```

```bash
./anitor-stream "My search term" --player "vlc"
```

## License

[GPL 3.0](LICENSE)


