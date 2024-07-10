# Download binaries
Are CI builds binaries for macOS apple silicon and linux.

## Curl
### MacOS
```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/essential-rest-server-macos-latest -o essential-rest-server && chmod 755 essential-rest-server && mkdir -p ~/.local/bin && mv -f essential-rest-server ~/.local/bin/essential-rest-server
```
```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/pint-macos-latest -o pint && chmod 755 pint && mkdir -p ~/.local/bin && mv -f pint ~/.local/bin/pint
```
### Linux
These are built on `ubuntu`.
```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/essential-rest-server-ubuntu-latest -o essential-rest-server && chmod 755 essential-rest-server && mkdir -p ~/.local/bin && mv -f essential-rest-server ~/.local/bin/essential-rest-server
```
```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/pint-ubuntu-latest -o pint && chmod 755 pint && mkdir -p ~/.local/bin && mv -f pint ~/.local/bin/pint
```

## Github
You can find the binaries as assets on the [latest release](https://github.com/essential-contributions/essential-integration/releases/latest) page.
> Due to macOS restrictions, the binaries will not run at first and you will need to right click on them and select open. \
After doing this once you will be able to run them in the terminal as normal.

> Warning: It's not a great idea to download binaries from the internet and run them on your machine. \
We recommend you use nix or build from source.

> We make an effort at keeping these binaries up to date but they may not be the latest version. \
To see which version a binary is you can check the `flake.lock` file on the commit tagged in the release.