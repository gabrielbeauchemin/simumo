/*!
Handle all CLI input for the simulator.
*/

use argparse::{ArgumentParser, Store};

///Wrap the command line arguments.
#[derive(Default)]
pub struct CommandLineArguments {
    pub configuration_file_path: String,
}

impl CommandLineArguments {
    ///This function will parse every options specify in CLI.
    pub fn parse() -> Self {
        let mut options = Self::default();
        {
            // this block limits scope of borrows by ap.refer() method
            let mut parser = ArgumentParser::new();
            parser.set_description("Command-line options");
            parser
                .refer(&mut options.configuration_file_path)
                .add_option(&["-c"], Store, "Json configuration file path")
                .required();

            parser.parse_args_or_exit();
        }
        options
    }
}
