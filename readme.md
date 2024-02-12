# VPK CFGY

Create or update many vpks with custom [regular expression](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions) file and path filtering.

## Configuration
On first run a default config is created. This can be editied to define many vpks to create.
```
{
    "vpk_path": "./bin/vpk.exe",
    "dir": "",
    "vpks": [
        {
            "regex": ".*",
            "dir_regex": ".*",
            "name": "all.vpk",
            "args": "-P"
        },
        {
            "regex": "(_low)$",
            "dir_regex": ".*",
            "name": "low.vpk",
            "args": "-P"
        }
    ]
}
```
- `vpk_path`: the path to a vpk executable from a source game.

- `dir`: the working directory (*optional*). If blank, the executable path will be used (or any path given as the first argument).

### vpk Configuration
`vpks` is an array of vpks to create. The properties are:
- `regex`: filters filenames (without extension)
- `dir_regex`: filters the file path, excluding is file name and extension.
- `name`: defines the name for this new vpk
- `args`: is an array of arguments to also use with the vpk executable (*optional*)

> [!TIP]
> Use https://regexr.com/ for help with building valid regular expressions. You can also refer to the [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions) or [python](https://docs.python.org/3/library/re.html) regex docs.

## Build
- Clone this repo
- Install rust tools via [rustup](https://rustup.rs/)
- Use ``cargo build --release`` to build