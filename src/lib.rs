use std::hash::{Hash, Hasher};//type Hash and method Hasher to hash
use std::collections::hash_map::DefaultHasher;//hashing algo
use std::fs::{self};
use std::io::{Read, BufReader};

#[derive(Debug)]
pub struct FileInfo{
    file_path_hash: Option<u64>,
    file_hash: Option<u64>,
    file_path: String
}

impl FileInfo{
    pub fn new(path_hash: Option<u64>, 
            hash: Option<u64>, path: String) -> Self{
        FileInfo{file_path_hash: path_hash,
            file_hash: hash, file_path: path}
    }

    pub fn get_path(&self) -> &str{
        self.file_path.as_str()
    }

    pub fn get_file_name(&self) -> &str{
        self.file_path.as_str().rsplit("/").next().unwrap()
    }

    pub fn get_hash(&self) -> Option<u64>{
        self.file_hash
    }

    pub fn get_path_hash(&self) -> Option<u64>{
        self.file_path_hash
    }

    pub fn set_hash(&mut self, hash: u64) -> () {
        self.file_hash = Some(hash)
    }

    pub fn set_path_hash(&mut self, hash: u64) -> () {
        self.file_path_hash = Some(hash)
    }

    pub fn generate_hash(&mut self) -> Option<u64>{
        let mut hasher = DefaultHasher::new();
        match fs::File::open(self.file_path.as_str().clone()){
            Ok(f) => {
                //read the file
                let mut buffer_reader = BufReader::new(f);
                let mut hash_buffer = [0; 4096];
                match buffer_reader.read(&mut hash_buffer) {
                        Ok(n) if n>0 => hasher.write(&hash_buffer[0..]),
                        Ok(n) if n==0 => (),
                        Err(e) => println!("{:?} reading {:?}", e, self.file_path.as_str()),
                        _ => println!("Should not be here"),
                }
                self.set_hash(hasher.finish());
                return self.get_hash()
            }
            Err(e) => {
                println!("Error:{} while opening {:?}. \
                Skipping.", e, self.file_path);
                return None
            }
        }
    }

    pub fn generate_path_hash(&mut self) -> Option<u64>{
        let mut hasher = DefaultHasher::new();
        let path = self.file_path.as_bytes().clone();
        hasher.write(path);
        self.set_path_hash(hasher.finish());
        return self.get_path_hash()
    }
}

impl PartialEq for FileInfo{
    fn eq(&self, other: &FileInfo) -> bool{
        (self.file_hash == other.file_hash)
    }
}

impl Eq for FileInfo{}

// impl PartialOrd for Fileinfo{
//     fn partial_cmp(&self, other: &Fileinfo) -> Option<Ordering>{
//         self.file_length.partial_cmp(&other.file_length)
//     }
// }

// impl Ord for Fileinfo{
//     fn cmp(&self, other: &Fileinfo) -> Ordering {
//         self.file_length.cmp(&other.file_length)
//     }
// }

impl Hash for FileInfo{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_hash.hash(state);
    }
}
