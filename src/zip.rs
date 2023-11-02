use flate2::read::GzDecoder;
use md5::compute;
use std::fs;
use std::path::Path;
use tar::Archive;
use walkdir::WalkDir;
use bzip2::read::BzDecoder;


pub fn process_gz_files(apath: String) {
    let gzlist = ["gz", "GZ"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
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
                    let outdir = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/GZ/".to_string()
                        + &fdigest;
                    let _out_dir = fs::File::create(outdir.clone()).unwrap();
                    let out_dir_path = Path::new(outdir.as_str());

                    a.unpack(out_dir_path).unwrap();
                };
            };
        };
    }
}
pub fn process_bz2_files(apath: String) {
    let bz2list = ["bz2", "BZ2"];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
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
                    let outdir = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BZ2/".to_string()
                        + &fdigest;
                    let _out_dir = fs::File::create(outdir.clone()).unwrap();
                    let out_dir_path = Path::new(outdir.as_str());

                    a.unpack(out_dir_path).unwrap();
                };
            };
        };
    }
}
