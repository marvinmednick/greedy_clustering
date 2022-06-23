extern crate clap;
use log::{ info, error, debug, warn,trace };

use clap::{Arg, Command};

#[derive(Debug)]
pub struct CommandArgs  {
    pub filename: String,
    pub num_clusters: usize,
    pub hamming: bool,
}

impl CommandArgs  {
    pub fn new() -> Self {
        // basic app information
        let app = Command::new("cluster")
            .version("1.0")
            .about("Determines clustering")
            .author("Marvin Mednick");

        // Define the name command line option
        let filename_option = Arg::new("file")
            .takes_value(true)
            .help("Input file name")
            .required(true);

        let clusters_option = Arg::new("clusters")
            .takes_value(true)
            .help("number of clusters")
            .required(true);

        let hamming_option = Arg::new("hamming")
            .takes_value(false)
            .help("cluster by hamming code");

        // now add in the argument we want to parse
        let mut app = app.arg(filename_option);
        app = app.arg(clusters_option);
        app = app.arg(hamming_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let filename = matches.value_of("file")
            .expect("Filename can't be None, we said it was required");

        let num_str = matches.value_of("clusters");

        let num_clusters = match num_str {
            None => { println!("Start is None..."); 0},
            Some(s) => {
                match s.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {println!("That's not a number! {}", s); 0},
                }
            }
        };

        let hamming = matches.is_present("hamming");

        info!("clap args: {} {} {}",filename, num_clusters, hamming);

        CommandArgs { filename: filename.to_string(), num_clusters : num_clusters, hamming: hamming }
    }   
}
