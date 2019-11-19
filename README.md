# dir2m3u

Create `.m3u` playlists from a directory structure

## Usage

```
dir2m3u 
Create m3u playlists from directories

USAGE:
    dir2m3u [FLAGS] <DIRECTORY>

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    recursive: turn subdirectories into playlists too
    -V, --version      Prints version information

ARGS:
    <DIRECTORY>    directory used to create the playlist [default: .]
```

Run the executable to create `m3u` playlist files with the songs in the directories you provided

Use `-r` to create a playlist for each directory contained in the one you selected.

All the `m3u` playlist files will have the same name as the directory they were created from

### Notes

Learning project
