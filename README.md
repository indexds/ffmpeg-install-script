# ffmpeg-install-script

ffmpeg-install-script fetches the latest ffmpeg build from [ffmpeg-builds](https://github.com/BtbN/FFmpeg-Builds/) and extracts it to ```C:\Program Files\ffmpeg```. You are given the option to add the ```.\ffmpeg\bin directory``` to PATH.

## Installation

To install, first pull the repository to your local machine using

```bash
git pull https://github.com/indexds/ffmpeg-install-script.git
```

Then, navigate to the repository

```bash
cd path/to/repository
```

And compile the source to an executable with

```bash
cargo build --release
```
## Usage

To use the script, simply run ```ffmpeg-install-script.exe``` and follow the instructions.
You may need to run the script as Administrator to allow PATH environment variable modification and extraction of the zip file.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
