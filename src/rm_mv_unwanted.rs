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
        }
    }
    println!("Start count: {}\nFiles removed: {}", idx, rmcount);
}

pub fn mv_vid_files(fname: String) {
    let mvlist = [
        "pdf", "PDF", "mp4", "MPG", "avi", "AVI", "mp3", "wav", "m4p", "m4a", "MP3", "zip", "ZIP",
        "gz", "bz2",
    ];

    let parts = &fname.split(".").collect::<Vec<&str>>();

    let ext = parts.last().unwrap();
    if mvlist.contains(&ext) {
        let fparts = fname.split("/").collect::<Vec<&str>>();
        let filename = fparts.last().unwrap().replace(" ", "_");
        let addr = "/media/pi/USB128/AV/".to_string() + &filename;
        match fs::rename(&fname, &addr) {
            Ok(_) => println!("Moved: {}", addr),
            Err(e) => println!("Error: {}", e),
        };
        println!("{:#?}", parts.last().unwrap())
    };
}
