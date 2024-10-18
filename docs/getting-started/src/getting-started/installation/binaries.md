# Download Binaries

Essential's CI configuration builds binaries for macOS (Apple Silicon) and Linux.

> **Warning**:  
> It's generally not recommended to download binaries from the internet and run them directly on your machine. **We highly suggest using Nix or building from source for a safer setup.**

## Download Using Curl

### macOS

To download and install the binaries on macOS, run the following commands in your terminal:

```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/essential-rest-server-macos-latest -o essential-rest-server && chmod 755 essential-rest-server && mkdir -p ~/.local/bin && mv -f essential-rest-server ~/.local/bin/essential-rest-server
```

```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/pint-macos-latest -o pint && chmod 755 pint && mkdir -p ~/.local/bin && mv -f pint ~/.local/bin/pint
```

> **macOS Users**:  
> Due to macOS restrictions, the binaries will not run at first. You will need to right-click on them and select "Open."  After doing this once, you will be able to run them in the terminal as normal.

---

### Linux

For Linux (built on Ubuntu), use the following commands to download and install the binaries:

```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/essential-rest-server-ubuntu-latest -o essential-rest-server && chmod 755 essential-rest-server && mkdir -p ~/.local/bin && mv -f essential-rest-server ~/.local/bin/essential-rest-server
```

```bash
curl -L https://github.com/essential-contributions/essential-integration/releases/latest/download/pint-ubuntu-latest -o pint && chmod 755 pint && mkdir -p ~/.local/bin && mv -f pint ~/.local/bin/pint
```

---

## Github Releases

You can find the binaries as assets on the [latest release](https://github.com/essential-contributions/essential-integration/releases/latest) page.

> **Note**:  
> We make an effort to keep these binaries up to date, but they may not always reflect the latest version. To check which version a binary is, you can review the `flake.lock` file on the commit tagged in the release.
