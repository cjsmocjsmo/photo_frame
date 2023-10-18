use std::fs;
use walkdir::WalkDir;

pub fn rm_unwanted_files(apath: String) {
    let mut idx = 0;
    let mut rmcount = 0;
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            // println!("fname: {}", fname);
            if fname.contains("System/Apps") {
                rmcount += 1;
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
            };
            // if fname.contains("openoffice") {
            //     rmcount += 1;
            //     println!("Removed: {}", &fname);
            //     fs::remove_file(&fname).unwrap();
            // };
            // if fname.contains("OpenOffice") {
            //     rmcount += 1;
            //     println!("Removed: {}", &fname);
            //     fs::remove_file(&fname).unwrap();
            // };
        }
    }
    println!("Start count: {}\nFiles removed: {}", idx, rmcount);
}
