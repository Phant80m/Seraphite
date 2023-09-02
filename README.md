 <div align='center'>

<h1>Seraphite</h1>
<p>a simple, but powerful configuration manager for linux systems!</p>

<h4> <span> · </span> <a href="https://github.com/phant80m/Seraphite/blob/master/README.md"> Documentation </a> <span> · </span> <a href="https://github.com/phant80m/Seraphite/issues"> Report Bug </a> <span> · </span> <a href="https://github.com/phant80m/Seraphite/issues"> Request Feature </a> </h4>


</div>

# :notebook_with_decorative_cover: Table of Contents

- [About the Project](#star2-about-the-project)
- [Roadmap](#compass-roadmap)
- [FAQ](#grey_question-faq)


## :star2: About the Project
⚠️  Warning: Some features only work on arch linux! ⚠️ 
### :dart: Features
- Supports multiple configs
- Blazingly fast!
- Super simple


## :toolbox: Getting Started

### :gear: Installation

How to install:
```bash
cargo install --git https://github.com/phant80m/seraphite
```
build manually
```bash
git clone https://github.com/phant80m/seraphite && cargo build --release
```
generate shell completions
```bash
seraphite --shell-completion <SHELL>
```

## :compass: Roadmap

* [x] Link files from dotfile dir to config dir
* [x] install dependencies from a file
* [x] Support multiple configs
* [x] post install script
* [x] easy setup


## :grey_question: FAQ

- Q: Does this work?
- A: Well, yes it does work and pretty well if i may add.
- Q: Is it hard to use
- A: no. It is incredibly simple to setup and use.
- Q: Can I use it with multiple configs
- A: Yes. you can specify the dotfile dir and the config folder
