use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use md5::compute;
use std::fs;
use std::path::Path;
use tar::Archive;
use walkdir::WalkDir;
use zip::ZipArchive;

pub fn process_gz_files() {
    let apath = "/media/pipi/0123-4567/GZ1".to_string();
    let gzlist = ["gz", "GZ"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.contains("python3-openid") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("torando") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("DO.NOT.DELETE") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("jqm-pagination-master") {
                std::fs::remove_file(fname.clone()).unwrap();
            } else {
                println!("processing gz file: \n{}", fname);
                let path = Path::new(&fname);
                if path.is_file() {
                    let digest = compute(fname.clone());
                    let fdigest = format!("{:?}", digest);
                    let parts = &fname.split(".").collect::<Vec<&str>>();
                    let ext = parts.last().unwrap();
                    if gzlist.contains(&ext) {
                        let tar = fs::File::open(fname.clone()).unwrap();
                        let dec = GzDecoder::new(tar);
                        let mut a = Archive::new(dec);
                        let outdir =
                            "/media/pipi/0123-4567/Images/GZ1_Unzip/".to_string() + &fdigest;
                        let _out_dir = fs::create_dir_all(outdir.clone()).unwrap();
                        let out_dir_path = Path::new(outdir.as_str());

                        a.unpack(out_dir_path).unwrap();
                        fs::remove_file(fname).unwrap();
                    };
                };
            };
        };
    }
}

pub fn process_zip_files() {
    let apath = "/media/pipi/0123-4567/ZIP".to_string();
    let ziplist = ["zip", "ZIP"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("processing zip file: \n{}", fname);
            let path = Path::new(&fname);
            if path.is_file() {
                let digest = compute(fname.clone());
                let fdigest = format!("{:?}", digest);
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if ziplist.contains(&ext) {
                    if fname.contains("Email_Photos.ZIP") {
                        fs::remove_file(fname.clone()).unwrap()
                    } else {
                        let mut archive =
                            ZipArchive::new(fs::File::open(fname.clone()).unwrap()).unwrap();
                        let outdir =
                            "/media/pipi/0123-4567/Images/ZIP_Unzip/".to_string() + &fdigest;
                        let _out_dir = fs::create_dir_all(outdir.clone()).unwrap();
                        let out_dir_path = Path::new(outdir.as_str());
                        for i in 0..archive.len() {
                            let mut file = archive.by_index(i).unwrap();
                            let outpath = match file.enclosed_name() {
                                Some(path) => out_dir_path.join(path.to_owned()),
                                None => continue,
                            };
                            if (&*file.name()).ends_with('/') {
                                fs::create_dir_all(&outpath).unwrap();
                            } else {
                                if let Some(p) = outpath.parent() {
                                    if !p.exists() {
                                        fs::create_dir_all(&p).unwrap();
                                    }
                                }
                                let mut outfile = fs::File::create(&outpath).unwrap();
                                std::io::copy(&mut file, &mut outfile).unwrap();
                            }
                        }
                    }
                    fs::remove_file(fname.clone()).unwrap();
                };
            };
        };
    }
}

pub fn process_bz2_files() {
    let apath = "/media/pipi/0123-4567/BZ2".to_string();
    let bz2list = ["bz2", "BZ2"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("processing bz2 file: \n{}", fname);
            let path = Path::new(&fname);
            if path.is_file() {
                let digest = compute(fname.clone());
                let fdigest = format!("{:?}", digest);
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if bz2list.contains(&ext) {
                    let tar = fs::File::open(fname.clone()).unwrap();
                    let dec = BzDecoder::new(tar);
                    let mut a = Archive::new(dec);
                    let outdir = "/media/pipi/0123-4567/Images/BZ2_Unzip/".to_string() + &fdigest;
                    let _out_dir = fs::create_dir_all(outdir.clone()).unwrap();
                    let out_dir_path = Path::new(outdir.as_str());

                    a.unpack(out_dir_path).unwrap();
                    fs::remove_file(fname).unwrap();
                };
            };
        };
    }
}
