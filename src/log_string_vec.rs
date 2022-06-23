use log::{ info, error, debug, warn,trace };

pub fn debug_vec(list_strings: Vec<String>) {
    for e in list_strings {
        debug!("{}",e);
    }
}

pub fn info_vec(list_strings: Vec<String>) {
    for e in list_strings {
        info!("{}",e);
    }
}
