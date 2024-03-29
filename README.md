# Code Evolution Insights

Application to aggregate info on historical VCS analysis based on strategies from the book Your Code As A Crime Scene

## Installation

To be able to run the application you need some dependencies on your environment

* [python programming language](https://www.python.org/downloads/)
* [rust programming language](https://www.rust-lang.org/tools/install)
* [code maat](https://github.com/adamtornhill/code-maat) project's exec should be added to your PATH, like ```export PATH=$PATH:"/Users/youruser/projects/code_maat/ixmaat0.8.5"```


## Usage

After installing the dependencies, in the project folder, execute ```cargo run -- --after YYYY-MM-DD --before YYYY-MM-DD --path <path-to-git-repo>``` to start the project.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
