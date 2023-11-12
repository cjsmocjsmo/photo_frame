use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn rm_unwanted_files(apath: String) -> i32 {
    let to_remove_list = [
        "0611160110.jpg",
        "0821161127a.jpg",
        "0821161127.jpg",
        "0821161128a.jpg",
        "0821161128.jpg ",
        "0821162314.jpg",
        "0822161108a.jpg",
        "0822161108.jpg",
        "0906141903a.jpg",
        "20150327_200735.jpg",
        "DSCN1120.JPG",
        "h150.jpg",
        "h151.jpg",
        "h152.jpg",
        "h153.jpg",
        "h154.jpg",
        "h155.jpg",
        "h156.jpg",
        "h157.jpg",
        "h158.jpg",
        "h159.jpg",
        "h15.jpg",
        "h160.jpg",
        "h161.jpg",
        "h162.jpg",
        "h163.jpg",
        "h164.jpg",
        "h165.jpg",
        "h166.jpg",
        "h167.jpg",
        "h168.jpg",
        "h169.jpg",
        "h16.jpg",
        "h170.jpg",
        "h171.jpg",
        "h172.jpg",
        "h173.jpg",
        "h17.jpg",
        "h18.jpg",
        "h19.jpg",
        "my_touch_phone_023.gif",
        "mytouchphone023.gif",
        "normallit_16_h.png",
        "normallit_16.png",
        "PART_1454822819828_20160206_131602.jpg",
        "phone_023.gif",
        "phone023.gif",
        "phone_270.gif",
        "phone270.gif",
        "phone276.jpg",
        ".mp3",
        ".wav",
        ".WAV",
    ];

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
            let tm = fname.split("/").collect::<Vec<&str>>();
            let filename = tm.last().unwrap();
            if fname.contains("System/Apps") {
                rmcount += 1;
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
            } else if fname.contains("python3-openid") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("torando") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("DO.NOT.DELETE") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("jqm-pagination-master") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("pussy") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else {
                println!("fuck")
            }

            for rm in to_remove_list {
                if filename.contains(rm) {
                    std::fs::remove_file(fname.clone()).expect("Unable to remove BadFile");
                }
            }

        }
    }
    println!("Start count: {}\nFiles removed: {}", idx, rmcount.clone());

    rmcount
}

pub fn mv_zip_files(fname: String) {
    let ziplist = ["zip", "ZIP", "gz", "GZ", "bz2", "BZ2"];

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
                } else if ext == &"bz2" || ext == &"BZ2" || ext == &"tar" || ext == &"TAR" {
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
        "pdf", "PDF", "mp4", "mpg", "MPG", "avi", "AVI",
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
            if ext == &"mp3" || ext == &"MP3" {
                std::fs::remove_file(fname.clone()).expect("Unable to remove BadFile");
            } else if mvlist.contains(&ext) {
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

pub fn rm_by_extension(apath: String) -> i32 {
    let rm_list = [
        "yaml",
        "py",
        "sql",
        "in",
        "rst",
        "sh",
        "cfg",
        "c",
        "csv",
        "mo",
        "po",
        "crt",
        "ini",
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
    println!("Start count: {}\nFiles removed: {}", count, rmcount.clone());

    rmcount.clone()
}
