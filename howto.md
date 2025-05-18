# Seraphite for Dummies

Welcome to **Seraphite** – a simple, yet powerful configuration manager for Linux systems!  
If you want to manage your dotfiles (config files) with ease, automate setup, and keep your stuff tidy, this tool is for you.  
This guide explains how to use Seraphite, step by step, with zero jargon.

---

## What is Seraphite?

Seraphite is a program that helps you manage, back up, and synchronize your configuration files (dotfiles) on Linux.  
**Warning:** Some features only work on Arch Linux!

### Features

- Supports multiple configurations ("configs")
- Super fast & easy to use
- Can link (symlink) your dotfiles to the right places
- Automates installing dependencies
- Lets you run post-install scripts
- Simple setup process

---

## How to Install Seraphite

There are two main ways to install Seraphite:

### 1. Easiest: Install Using Cargo

If you have Rust and Cargo installed:

```bash
cargo install --git https://github.com/phant80m/seraphite
```

### 2. Building Manually

If you prefer to build from source:

```bash
git clone https://github.com/phant80m/seraphite
cd seraphite
cargo build --release
```

### 3. Generate Shell Completions (Optional)

To make your terminal auto-complete Seraphite commands:

```bash
seraphite --shell-completion <SHELL>
# Replace <SHELL> with bash, zsh, fish, etc.
```

---

## Quick Start: Your First Time Setup

### Step 1: Create Your Dotfiles Directory

Run:

```bash
seraphite setup
```

This creates a `dotfiles` directory in your home folder.  
**Add your config files** (like `.bashrc`, `.vimrc`, etc.) to `~/dotfiles/.config/`.

### Step 2: Link Your Dotfiles

Once your configs are ready:

```bash
seraphite tether
```

This links (symlinks) your configs from `~/dotfiles/.config/` into your actual `~/.config/` directory.

- Want to use a different dotfiles directory or config?  
  Use `--dot-dir` and `--config` options.

### Step 3 (Optional): Install Dependencies

If you have a `dependencies` file in your dotfiles folder listing needed packages:

```bash
seraphite sync
```

This will install all dependencies using `paru` (an AUR helper for Arch).

### Step 4 (Optional): Run Post-Install Scripts

If you want to automatically run an install script (for extra setup):

```bash
seraphite enchant --shell bash
```

Replace `bash` with your preferred shell.  
Make sure you have a `post_install.sh` script in your `dotfiles` directory.

---

## Common Commands Cheat Sheet

| Command                            | What It Does                                                     |
|-------------------------------------|------------------------------------------------------------------|
| `seraphite setup`                   | Sets up a new dotfiles folder                                    |
| `seraphite tether`                  | Links your dotfiles to your config directory                     |
| `seraphite untether`                | Unlinks your dotfiles                                            |
| `seraphite sync`                    | Installs dependencies from `dotfiles/dependencies`               |
| `seraphite enchant --shell bash`    | Syncs, tethers, and runs post-install script using given shell   |
| `seraphite docs`                    | Shows dotfile documentation (if available)                       |
| `seraphite clone <url> [branch]`    | Clones a dotfiles repo to `~/dotfiles`                           |

---

## FAQ

- **Does this work?**  
  Yes, and pretty well if I may add.

- **Is it hard to use?**  
  No. It is incredibly simple to set up and use.

- **Can I use it with multiple configs?**  
  Yes! You can specify the dotfile directory and the config folder.

---

## Troubleshooting

- **"Paru not found" error:**  
  Seraphite uses `paru` to install dependencies. If you don't have it, Seraphite will try to install it for you.  
  If that fails, install `paru` manually or on a supported Arch Linux system.

- **"Dependency file not found":**  
  Make sure you have a `dependencies` file in your `dotfiles` folder if you want to use `seraphite sync`.

- **Backups:**  
  When linking, Seraphite can create a backup of your original configs. Always read prompts before confirming.

---

## Pro Tips

- Keep your dotfiles in version control (like git) for extra safety.
- Use the `clone` command to copy dotfiles from another repo.
- You can unlink with `seraphite untether` if you want to remove symlinks.

---

## Still Lost?

Check the official documentation:  
[Seraphite Documentation](https://github.com/phant80m/Seraphite/blob/master/README.md)  
Or open an issue for help!

---

Enjoy managing your configs, the easy way! 🚀
