extern crate clap;
extern crate walkdir;

use clap::{App, Arg};
use walkdir::WalkDir;
use std::fmt::Debug;//Help in printing HashMap
use std::hash::Hash;//Help in printing HashMap
use std::fs::metadata;//read the meta data to check for dir vs file
use std::collections::HashMap;

extern crate dupper;
use dupper::{FileInfo};

fn main() {
    //clap method to accept teminal arguments.App
    let arguments = App::new("Duplucate Finder")
        .version("v0.1")
        .author("Clament John")
        .about("Find duplicate files in your filesystem")
        .arg(Arg::with_name("directories")
                               .short("d")
                               .long("directories")
                               .value_name("Directories")
                               .help("Directories to parse")
                               .min_values(1)
                               .required(true)
                               .takes_value(true)
                               .index(1))
        .get_matches();

    //get the directory(s) passed in. We use "values_of" to collect more than one arg
    let search_dirs: Vec<_> = arguments.values_of("directories").unwrap().collect();

    let mut file_counter: HashMap<Option<u64>, u64> = HashMap::new();
    let mut file_info: HashMap< Option<u64>, FileInfo> = HashMap::new();
    //read the file paths all the way upto individual files
    for dir in search_dirs.iter(){
        for e in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if metadata(e.path().display().to_string()).unwrap().is_file(){
                let mut file_obj = FileInfo::new(None, None, e.path().display().to_string() );
                file_obj.generate_hash();
                file_obj.generate_path_hash();

                // println!("File Name: {:?}\nPath Hash: {:?}\nHash: {:?}\n",
                // file_obj.get_file_name(), file_obj.get_path_hash(), file_obj.get_hash());

                *file_counter.entry( file_obj.get_hash() ).or_insert(0) += 1;
                file_info.insert(file_obj.get_path_hash(), file_obj);
            }
        }

        print_map(&file_counter);
        print_map(&file_info);

    }
}

fn print_map<K: Debug + Eq + Hash, V: Debug>(map: &HashMap<K, V>) {
    for (k, v) in map.iter() {
        println!("{:?}: {:#?}", k, v);
    }
}