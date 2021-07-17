# anitor-stream

Search and live stream anime/hentai torrent from the cli.

## Installation

For this script to work you need to have these dependencies installed:
```bash
webtorrent-cli
mpv
```

After you have installed them you can clone this repo:
```console
git clone https://github.com/gan-of-culture/anitor-stream.git
```

Then move into the directory and make the script executable:
```bash
cd anitor-stream && chmod +x anitor-stream
```

## Usage

```bash
./anitor-stream [SEARCH_QUERY]
```
Replace the ```SEARCH_QUERY``` with the name of the show and episode you wanna watch. For example:

```bash
./anitor-stream Mahouka Koukou no Yuutousei
```

After that you can enter the torrent category. For Eng-Subs choose the first opiton. Now up two 74 available torrents will be listed.
Enter the number of the torrent you want to watch and press enter. Now the video should start playing.

## TODOs

I'll probably give the user the options to choose the video player at a later stage via a config file.

## License

[GPL 3.0](LICENSE)


