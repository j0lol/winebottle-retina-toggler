# wine bottle retina toggler

Reads or writes display settings to your wine bottle's registry. Don't pass in `-d` or `-r` to read the values.
```
Usage: winebottle_retina_toggler [OPTIONS] <BOTTLE_DIRECTORY>

Arguments:
  <BOTTLE_DIRECTORY>  

Options:
  -d, --dpi <DPI>        
  -r, --retina <RETINA>  [possible values: true, false]
  -w, --write-to-test    
  -h, --help             Print help
```

## I have no idea what this is how do I download it.

You are in a Rust project. To run it, build and run the program with [cargo](https://rustup.rs). Alternatively, I might have published a build in the [Releases section](https://github.com/j0lol/winebottle-retina-toggler/releases/latest). You will probably have to unzip and clear the quarantine flag: `xattr -c ./file`.
