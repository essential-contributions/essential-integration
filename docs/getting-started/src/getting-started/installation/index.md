# Installation

To start building Pint applications, you'll need to install a few essential tools. Below are three installation options, depending on your preference:

### 1. **Using Nix** (Recommended)

The easiest and most convenient method is to use the [Nix package manager](nix.md). Nix automatically handles dependencies, making setup hassle-free. Follow the instructions in the Nix installation guide to get started.

### 2. **Installing from crates.io**

You can install the Essential tools directly from crates.io using Cargo. This method allows you to easily install prebuilt tools, though they are built from source by Cargo. To use this method, ensure you have Cargo installed on your system. Detailed instructions can be found [here](source.md).

---

## Additional Setup

### Rust Installation

If you are using `nix`, you can simply run the following command to launch a dev shell with all the necessary tools installed:

```bash
nix develop github:essential-contributions/essential-integration
```

If you don't have Rust and Cargo installed, follow the official installation instructions [here](https://www.rust-lang.org/tools/install).

### Syntax Highlighting

To improve the development experience, we are adding syntax highlighting support for Pint across popular code editors. Currently, support is available for the following editor:

#### Visual Studio Code (VSCode)

To enable syntax highlighting for Pint in VSCode, you can search for `pint syntax` in the marketplace or use this [link](https://marketplace.visualstudio.com/items?itemName=essential-contributions.pint-lang).
