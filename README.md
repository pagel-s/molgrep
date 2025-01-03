[![Conference][contributors-shield]][contributors-url]
[![Stargazers][stars-shield]][stars-url]


# molgrep

Small Rust CLI interface to search for molecules with matching substructures from the command line. 

molgrep is built on top of [rdkitcffi](https://github.com/chrissly31415/rdkitcffi).<br/>
Currently directly searching .txt (one smiles per line) and SDF-files is supported ([see examples](#examples)).<br/><br/>


 ## Examples

For searching .txt files/ files with a single smiles string per entry
 ```cli
 molgrep --smarts c1ccccc1 --path ./smiles.txt
 ```
A example input file with 1000 molecules is provided in smiles.txt
 <br/><br/>

To search SDF files just add the file-type flag 
 ```cli
 molgrep --smarts c1ccccc1 --path ./smiles.sdf --file-type sdf
 ```


## Prerequisites:  
<b>NOTE:</b> [rdkitcffi](https://github.com/chrissly31415/rdkitcffi) currently only supports linux , molgrep will thus also only work on linux.   

* [rust](https://www.rust-lang.org/tools/install)
* [rdkitcffi](https://github.com/chrissly31415/rdkitcffi?tab=readme-ov-file#installation)

## Installation

Download the repo:  

```
git clone https://github.com/pagel-s/molgrep.git  
```

If you have a rust/cargo installation, just run

```
cd molgrep
cargo build  
cargo test --lib  
```

## Usage

To execute the `molgrep` program from anywhere, you need to add the compiled binary to your system's PATH. After building the project, you can do this by running:

```sh
export PATH=$PATH:/PATH/TO/REPO/molgrep/target/debug
```

You can also add this line to your shell's configuration file (e.g., `.bashrc` or `.zshrc`) to make the change permanent.



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/pagel-s/molgrep.svg?style=for-the-badge
[contributors-url]: https://github.com/pagel-s/molgrep/graphs/contributors
[stars-shield]: https://img.shields.io/github/stars/croningp/molnca.svg?style=for-the-badge
[stars-url]: https://github.com/croningp/molca/stargazers
