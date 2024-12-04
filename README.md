<h1 align="center">Tor Onion V3 Address Generator</h1>

<p align="center">
    <img src="https://api.visitorbadge.io/api/visitors?path=https%3A%2F%2Fgithub.com%2Fjoshuavanderpoll%2FOnion-V3-Generator&label=Views&countColor=%2337d67a" />
    <a href="https://www.python.org/">
      <img src="https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54" alt="Python">
    </a>
</p>

## 📜 Description
This project is a Python script for generating `.onion` V3 addresses for Tor services. It allows users to create random addresses or addresses with a specified prefix. The script supports generating a fixed number of addresses or running indefinitely for continuous address generation.

## 🛠️ Installation
> [!NOTE]
> To ensure a clean and isolated environment for the project dependencies, it's recommended to use Python's `venv` module.

### iOS/Linux
```bash
git clone https://github.com/joshuavanderpoll/Onion-V3-Generator.git
cd Onion-V3-Generator
python3 -m venv .venv
source .venv/bin/activate
pip3 install -r requirements.txt
```

### Windows
```bash
git clone https://github.com/joshuavanderpoll/Onion-V3-Generator.git
cd Onion-V3-Generator
python -m venv .venv 
.venv\Scripts\activate
pip3 install -r requirements.txt
```

## ⚙️ Usage

To generate domains which start with "github" or "example" use:
```bash
python3 onion_generator github example
```

## 💡 Contributing to the project
To contribute, first fork this repository, and `clone` it. Make your changes, whether you're fixing bugs or adding features. When done, `commit` your changes, `push` them, and submit a `pull request` for review to this repository.

### Issues
If you're reporting an issue, make sure to include your `Python version` (python --version), and any relevant command input, and output.
