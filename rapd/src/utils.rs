use std::path::Path;

pub fn file_exists(path: String) -> bool {
    if Path::new(&path).exists(){
        return true;
    } else {
        return false;
    }
}
