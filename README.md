# Code Evolution Insights

Application to aggregate info on historical VCS analysis based on strategies from the book Your Code As A Crime Scene

## Installation

To be able to run the application you need some dependencies on your environment

* [python programming language](https://www.python.org/downloads/)
* [rust programming language](https://www.rust-lang.org/tools/install)
* [rustup installer](https://rustup.rs/) to be able to change between rust release channels. As we are using the [rocket framework](https://rocket.rs/) to publish pages we need to use rust nightly release channel
  * after rustup installation use 
  ```rustup default nightly``` to change all your rust projects to nightly channel, or ```rustup override set nightly``` in the project folder to change it only for this project
 * [code maat](https://github.com/adamtornhill/code-maat) project's exec should be added to your PATH, like ```export PATH=$PATH:"/Users/youruser/projects/code_maat/ixmaat0.8.5"```


## Usage

After installing the dependencies, in the project folder, execute ```cargo run``` to start the project.
* The path to the project to be analysed is still hardcoded in main.rs, as well as the analisys period. Need to be extracted as arguments

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
