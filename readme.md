# VPK CFGY

Create or update many vpks with custom [regular expression](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions) file name and path filtering.

## Configuration
On first run a default config in JSON is created. This can be editied to define many vpks to create.
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
- `vpk_path`: the path to a vpk executable from a source game. (e.g. `"C:\\SteamLibrary\\steamapps\\common\\Half-Life 2\\bin\\vpk.exe"`)

- `dir`: the starting directory to look for files in (*optional*).

### vpk Configuration
`vpks` is an array of vpks to create. The properties are:
- `regex`: filters filename + extension.
- `dir_regex`: filters the file path, excluding is file name and extension.
- `name`: defines the name for this new vpk
- `args`: array of arguments to also send to the vpk executable [e.g. `"-P"`] (*optional*)

> [!TIP]
> Use https://regexr.com/ for help with building valid regular expressions. You can also refer to the [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions) or [python](https://docs.python.org/3/library/re.html) regex docs.

> [!WARNING]
> For some expressions to be valid in the JSON config you will need to escape foward slashes (\)
>
> e.g. Expression to match a *.vtf* extension only `\.vtf` should be `"\\.vtf"` in a JSON string.

## Launch Arguments

Starting directory can be given as the first argument to override it. E.g. `./vpk_cfgy.exe "C:/Users/Rob/stuff"`

## Build
- Clone this repo
- Install rust tools via [rustup](https://rustup.rs/)
- Use ``cargo build --release`` to build