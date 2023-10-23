use md5::compute;
// use std::fs;
// use std::{path::PathBuf, str::FromStr};
use std::path::Path;
use walkdir::WalkDir;
use image::GenericImageView;
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
    pub fn resize_landscape_image(&self) {
        let image = image::open(self.path.clone()).unwrap();
        let img_dims = image.dimensions();
        let aspect = img_dims.0 as f64 / img_dims.1 as f64;

        let (newwidth, newheight) = calc_new_landscape_dims(image.width() as f64, image.height() as f64, aspect);

        let resized = image.resize(newwidth as u32, newheight as u32, image::imageops::FilterType::Lanczos3);
        let _save_image = resized.save(self.create_outfile()).unwrap();
    }
}

pub fn calc_new_landscape_dims(oldwidth: f64, oldheight: f64, aspect: f64) -> (f64, f64) {
    let mut newwidth = 0.0;
    let mut newheight = 0.0;
    if oldwidth > 800.0 as f64 {
        newwidth = 800.0 as f64;
        newheight = 800.0 as f64 / aspect.clone();
    } else if oldwidth < oldheight {
        println!("portrait");
        if oldheight > 800.0 as f64 {
            newheight = 800.0 as f64;
            newwidth = 800.0 as f64 / aspect.clone();
        };
    } else {
        println!("square");
    };
    println!(
        "width: {}\nheight: {}\naspect_ratio: {}\nnewwidth: {}\nnewheight: {}\n",
        oldwidth, oldheight, aspect, newwidth, newheight
    );

    (newwidth, newheight)
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
