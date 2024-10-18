# Installation

To start building Pint applications, you will need to install a few essential tools. Below are three installation options, depending on your preference:

### 1. **Using Nix** (Recommended)

The easiest and most convenient method is to use the [Nix package manager](nix.md). Nix automatically handles dependencies, making setup hassle-free. Follow the instructions provided in the Nix installation guide to get started.

### 2. **Download Precompiled Binaries**

If you want to avoid installing dependencies manually, you can directly download precompiled binaries for your platform. This is the fastest way to get up and running. Access the latest binaries [here](binaries.md).

### 3. **Building from Source**

For those who prefer full control over the setup process, you can opt to build Pint from the source. This method gives you maximum flexibility but requires more setup steps. Detailed instructions for building from source can be found [here](source.md).

### Additional Setup

- **Rust Installation**:  
 
    If you don't have Rust and Cargo installed you can follow the instructions [here](https://www.rust-lang.org/tools/install).

    <details>
    <summary>If you are using Nix you can follow these instructions to get Rust.</summary>

    If you are using `nix` you can simply run the following command to launch a dev shell with all the necessary tools installed:
    ```bash
    nix develop github:essential-contributions/essential-integration#dev
    ```

    Or you can create your own `flake` using:
    ```bash
    cd counter
    nix flake init -t github:essential-contributions/essential-integration
    nix develop
    ```
    </details>

  
- **Syntax Highlighting**:  
  For better development experience, enable Pint syntax highlighting in your code editor. See the [Syntax Highlighting](#syntax-highlighting) section below for more details.