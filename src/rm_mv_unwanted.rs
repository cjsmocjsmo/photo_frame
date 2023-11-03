use std::fs;
use walkdir::WalkDir;
use std::path::Path;


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
            if fname.contains("System/Apps") {
                rmcount += 1;
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
            } else if fname.contains("python3-openid") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("torando") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("DO.NOT.DELETE") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("jqm-pagination-master") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else {
                println!("fuck")
            }
        }
    }
    println!("Start count: {}\nFiles removed: {}", idx, rmcount);
}

pub fn mv_zip_files(fname: String) {
    let ziplist = [
        "zip", "ZIP", "gz", "GZ", "bz2", "BZ2",
    ];

    // let save_dir = Path::new("/media/pipi/0123-4567/AV/");
    // if !fs::metadata(save_dir).unwrap().is_dir() {
    //     fs::create_dir(save_dir).unwrap();
    // }

    for e in WalkDir::new(fname)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if ziplist.contains(&ext) {
                if ext == &"zip" || ext == &"ZIP" {
                    let fparts = fname.split("/").collect::<Vec<&str>>();
                    let filename = fparts.last().unwrap().replace(" ", "_");
                    let addr = "/media/pipi/0123-4567/ZIP/".to_string() + &filename;
                    match fs::rename(&fname, &addr) {
                        Ok(_) => println!("Moved: {}", addr),
                        Err(e) => println!("ZIP_Error: {}", e),
                    };
                } else if ext == &"gz" || ext == &"GZ" {
                    let fparts = fname.split("/").collect::<Vec<&str>>();
                    let filename = fparts.last().unwrap().replace(" ", "_");
                    let addr = "/media/pipi/0123-4567/GZ1/".to_string() + &filename;
                    match fs::rename(&fname, &addr) {
                        Ok(_) => println!("Moved: {}", addr),
                        Err(e) => println!("GZ_Error: {}", e),
                    };
                } else if ext == &"bz2" || ext == &"BZ2" {
                    let fparts = fname.split("/").collect::<Vec<&str>>();
                    let filename = fparts.last().unwrap().replace(" ", "_");
                    let addr = "/media/pipi/0123-4567/BZ2/".to_string() + &filename;
                    match fs::rename(&fname, &addr) {
                        Ok(_) => println!("Moved: {}", addr),
                        Err(e) => println!("BZ_Error: {}", e),
                    };
                }


                println!("Moved: {}", fname.clone())
            };
        }
    }
}

pub fn mv_vid_files(fname: String) {
    let mvlist = [
        "pdf", "PDF", "mp4", "mpg", "MPG", "avi", "AVI", "mp3", "MP3", "wav", "WAV",
    ];

    let save_dir = Path::new("/media/pipi/0123-4567/AV/");
    if !fs::metadata(save_dir).unwrap().is_dir() {
        fs::create_dir(save_dir).unwrap();
    }

    for e in WalkDir::new(fname)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if mvlist.contains(&ext) {
                let fparts = fname.split("/").collect::<Vec<&str>>();
                let filename = fparts.last().unwrap().replace(" ", "_");
                let addr = "/media/pipi/0123-4567/AV/".to_string() + &filename;
                match fs::rename(&fname, &addr) {
                    Ok(_) => println!("Moved: {}", addr),
                    Err(e) => println!("Error: {}", e),
                };
                println!("{:#?}", parts.last().unwrap())
            };
        }
    }
}

pub fn rm_by_extension(apath: String) -> bool {
    let rm_list = [
        "m4p",
        "m4a",
        "key",
        "htm",
        "txt",
        "ps1",
        "jar",
        "dat",
        "3gp",
        "nfo",
        "m3u",
        "jpgblk",
        "THM",
        "torrent",
        "info",
        "epp",
        "db",
        "mix",
        "xml",
        "doc",
        "itl",
        "ssf",
        "bak",
        "ctl",
        "lnk",
        " SF",
        "exe",
        "thm",
        "docx",
        "js",
        "css",
        "html",
        "colorstarmutedjpg",
        "redsheartsswirljpg",
        "redyucaflowerjpg",
        "wrapwoodjpg",
    ];
    let mut count = 0;
    let mut rmcount = 0;

    for e in WalkDir::new(&apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            count += 1;
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();

            let ext = parts.last().unwrap();
            if rm_list.contains(&ext) {
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
                rmcount += 1;
            };
        };
    }
    println!("Start count: {}\nFiles removed: {}", count, rmcount);

    true
}
