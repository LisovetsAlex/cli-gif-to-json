# Cli Gif to Json
Uses `gif-for-cli` to read output gif in cli and save each frames data to json.

Requires `gif-for-cli` installed.

# Execute
```
cli-gif-to-json --input /path/to/file.gif --output /path/to/output.json --character "#" --rows 15 --cols 30 --max_frames 100
--input: path to gif
--ouput: path to created json
--character: character that will be used as "pixel"
--rows: how many rows should the gif have
--cols: how many column should the gif have
--max_frames: maximum number of frames to save
```

# How to compile
```
cargo build --release

sudo cp target/release/cli-gif-to-json /usr/local/bin
```

