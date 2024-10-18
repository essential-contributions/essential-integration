# Installation

To start building Pint applications, you'll need to install a few essential tools. Below are three installation options, depending on your preference:

### 1. **Using Nix** (Recommended)

The easiest and most convenient method is to use the [Nix package manager](nix.md). Nix automatically handles dependencies, making setup hassle-free. Follow the instructions in the Nix installation guide to get started.

### 2. **Download Binaries**

If you want to avoid installing dependencies manually, you can directly download precompiled binaries for your platform. This is the fastest way to get up and running. Access the latest binaries [here](binaries.md).

### 3. **Building from Source**

For those who prefer full control over the setup process, you can opt to build Pint from the source. This method offers maximum flexibility but requires more setup steps. Detailed instructions for building from source can be found [here](source.md).

---

## Additional Setup

### Rust Installation

If you are using `nix`, you can simply run the following command to launch a dev shell with all the necessary tools installed:

```bash
nix develop github:essential-contributions/essential-integration#dev
```

If you don't have Rust and Cargo installed, follow the official installation instructions [here](https://www.rust-lang.org/tools/install).

### Syntax Highlighting

To improve the development experience, we are adding syntax highlighting support for Pint across popular code editors. Currently, support is available for the following editor:

#### Visual Studio Code (VSCode)

To enable syntax highlighting for Pint in VSCode, you can search for `pint syntax` in the marketplace or use this [link](https://marketplace.visualstudio.com/items?itemName=essential-contributions.pint-lang).