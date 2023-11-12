use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn rm_unwanted_files(apath: String) -> i32 {
    // let to_remove_list = [
    //     // "0611160110.jpg",
    //     // "0821161127a.jpg",
    //     // "0821161127.jpg",
    //     // "0821161128a.jpg",
    //     // "0821161128.jpg ",
    //     // "0821162314.jpg",
    //     // "0822161108a.jpg",
    //     // "0822161108.jpg",
    //     // "0906141903a.jpg",
    //     // "20150327_200735.jpg",
    //     // "DSCN1120.JPG",
    //     // "h150.jpg",
    //     // "h151.jpg",
    //     // "h152.jpg",
    //     // "h153.jpg",
    //     // "h154.jpg",
    //     // "h155.jpg",
    //     // "h156.jpg",
    //     // "h157.jpg",
    //     // "h158.jpg",
    //     // "h159.jpg",
    //     // "h15.jpg",
    //     // "h160.jpg",
    //     // "h161.jpg",
    //     // "h162.jpg",
    //     // "h163.jpg",
    //     // "h164.jpg",
    //     // "h165.jpg",
    //     // "h166.jpg",
    //     // "h167.jpg",
    //     // "h168.jpg",
    //     // "h169.jpg",
    //     // "h16.jpg",
    //     // "h170.jpg",
    //     // "h171.jpg",
    //     // "h172.jpg",
    //     // "h173.jpg",
    //     // "h17.jpg",
    //     // "h18.jpg",
    //     // "h19.jpg",
    //     // "my_touch_phone_023.gif",
    //     // "mytouchphone023.gif",
    //     // "normallit_16_h.png",
    //     // "normallit_16.png",
    //     // "PART_1454822819828_20160206_131602.jpg",
    //     // "phone_023.gif",
    //     // "phone023.gif",
    //     // "phone_270.gif",
    //     // "phone270.gif",
    //     // "phone276.jpg",
    // ];

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
            // let filename = tm.last().unwrap();
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
            } else if fname.contains("0611160110.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0821161127a.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0821161127.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0821161128a.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0821161128.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0821162314.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0822161108a.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0822161108.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("0906141903a.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("20150327_200735.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("DSCN1120.JPG") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h150.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h151.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h152.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h153.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h154.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h155.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h156.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h157.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h158.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h159.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h15.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h160.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h161.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h162.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h163.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h164.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h165.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h166.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h167.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h168.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h169.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h16.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h170.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h171.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h172.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h173.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h17.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h18.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("h19.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("my_touch_phone_023.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("mytouchphone023.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("normallit_16_h.png") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("normallit_16.png") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("PART_1454822819828_20160206_131602.jpg") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("phone_023.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("phone023.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("phone_270.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("phone270.gif") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else {
                println!("fuck")
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
                } else if ext == &"tar" {
                    let fparts = fname.split("/").collect::<Vec<&str>>();
                    let filename = fparts.last().unwrap();
                    if fname == "P-009.tar" {
                        std::fs::remove_file(fname.clone()).unwrap();
                    } else {
                        let addr = "/media/pipi/0123-4567/BZ2/".to_string() + &filename;
                        match fs::rename(&fname, &addr) {
                            Ok(_) => println!("Moved: {}", addr),
                            Err(e) => println!("BZ_Error: {}", e),
                        };
                    }
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
        "mp3",
        "MP3",
        "wav",
        "WAV",
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
