pub mod split_by_size {
    use clap::ArgMatches;

    pub fn split_file_by_size ( arguments: std::option::Option<&ArgMatches>, _sep: &str ) {
        let fasta_file: &String = arguments.unwrap().get_one::<String>("fasta-file").unwrap();
        println!("Fasta file for processing: {:?}", &fasta_file);
        println!("Size to chunk fasta into: {:?}", arguments.unwrap().get_one::<u16>("mem-size").unwrap());
    }

} 

