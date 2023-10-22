use md5::compute;
// use std::fs;
// use std::{path::PathBuf, str::FromStr};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Factory {
    pub path: String,
}

impl Factory {
    pub fn create_outfile(&self) -> String {
        let digest = compute(&self.path);
        let a = "/media/pi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/".to_string();
        let b = format!("{:?}", digest) + ".jpg";
        let newfilename = a + &b;

        newfilename
    }

    pub fn create_rename_output_file(&self) -> String {
        let fparts = self.path.split(".").collect::<Vec<&str>>();
        let filename = fparts.first().unwrap().replace(" ", "_");
        let addr = "/media/pi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/".to_string()
            + &filename
            + ".jpg";

        addr
    }
}

pub fn gen_ext_list(apath: String) -> Vec<String> {
    let mut ext_list: Vec<String> = Vec::new();
    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path();
            if let Some(extension) = fname.extension() {
                let ext = extension.to_owned().to_str().unwrap().to_string();
                if !ext_list.contains(&ext) {
                    ext_list.push(ext);
                };
            };
        }
    }

    ext_list
}

pub fn convert_image_to_jpg(apath: String) {
    let apath = Path::new(apath.as_str());
    let new_filename = apath.file_name().unwrap().to_str().unwrap().to_owned() + ".jpg";
    let new_path = apath.parent().unwrap().join(&new_filename);
    new_path.to_string_lossy().to_string();
    let image = image::open(apath).unwrap();
    let _save_image = image.save(new_path).unwrap();
}
