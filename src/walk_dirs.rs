// use crate::setup::rusic_walk_dirs;
// use std::env;
// use image::*;
// use image::{DynamicImage, ImageBuffer, ImageFormat};
use md5::compute;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
// use self::webp::Encoder;
// use self::webp::WebPMemory;

// use std::fs::File;
// use std::io::Write;

pub fn walk_dir(apath: String) -> Vec<String> {
    // let ext_vec: Vec<String> = Vec::new();
    let mut keeper_vec = Vec::new();
    let mut idx = 0;
    let rmlist = [
        "zip", "tar", "pdf", "torrent", "3gp", "info", "epp", "THM", "db", "mix", "txt", "xml",
        "log", "INI", "dat", "dll", "chm", "u3p", "inf", "swa", "cct", "lang", "master", "lck",
        "exe", "html", "bat", "ini", "py", "lst", "rdb", "jar", "bin", "js", "pyd", "uue", "out",
        "au", "doc", "icns", "pyw", "def", "xpt", "res", "com", "class", "xsl", "cfg", "key", "ht",
        "TAB", "tree", "css", "xlc", "xba", "xlb", "bau", "fmt", "soe", "soc", "soh", "sog", "sod",
        "sob", "thm", "sdv", "odb", "dbf", "dbt", "dic", "map", "xcu", "sample", "xcs", "htm",
        "sxw", "stw", "otp", "ott", "wmf", "ots", "sdg", "xdl", "pl", "asp", "inc", "idx", "aff",
        "dtd", "mod", "java", "bsh", "nsi", "ico", "yos", "gps", "CSS", "itl", "DS_Store", "ssf",
        "bak", "ctl", "lnk", " SF", "jpgblk", "ps1", "m3u", "nfo",
    ];
    // let mvlist = [
    //     "pdf", "PDF", "mp4", "MPG", "avi", "AVI", "mp3", "wav", "m4p", "m4a", "MP3", "zip", "ZIP",
    //     "gz", "bz2",
    // ];
    let keeplist = [
        "jpg", "JPG", "bmp", "BMP", "gif", "png", "tif", "jpeg", "PNG", "GIF",
    ];



    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            // println!("fname: {}", fname);
            // if fname.contains("System/Apps") {
            //     fs::remove_file(&fname).unwrap();
            // };
            // if fname.contains("openoffice") {
            //     fs::remove_file(&fname).unwrap();
            // };
            // if fname.contains("OpenOffice") {
            //     fs::remove_file(&fname).unwrap();
            // };

            let path = Path::new(&fname);

            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();

                let ext = parts.last().unwrap();
                // println!("ext: {}", ext);

                if rmlist.contains(&ext) {
                    match fs::remove_file(&fname) {
                        Ok(_) => println!("Removed: {}", &fname),
                        Err(e) => println!("Error: {}", e),
                    };
                };
                // if mvlist.contains(&ext) {
                //     let fparts = fname.split("/").collect::<Vec<&str>>();
                //     let filename = fparts.last().unwrap().replace(" ", "_");
                //     let addr = "/media/pi/USB128/AV/".to_string() + &filename;
                //     match fs::rename(&fname, &addr) {
                //         Ok(_) => println!("Moved: {}", addr),
                //         Err(e) => println!("Error: {}", e),
                //     };
                //     println!("{:#?}", parts.last().unwrap())
                // };

                // let mut keeper_vec = Vec::new();
                if keeplist.contains(&ext) {
                    // println!("fname: {}", fname);
                    keeper_vec.push(fname.clone());
                    // let oldfilename = fname.clone();
                    // println!("oldfilename {}", oldfilename);

                    // let mut newwidth = 0.0;
                    // let mut newheight = 0.0;
                    // if oldwidth < 200.0 as f64 {
                    //     let old_fn = fname.clone();
                    //     let fn_parts = old_fn.split("/").collect::<Vec<&str>>();
                    //     let fnam = fn_parts.last().unwrap();
                    //     let new_fn = "/media/pi/USB128/SmallPics/".to_string() + fnam;
                    //     match fs::rename(&fname, &new_fn) {
                    //         Ok(_) => println!("Moved: {}", new_fn),
                    //         Err(e) => println!("Error: {}", e),
                    //     };
                    // } else if oldwidth > oldheight {
                    //     println!("landscape");
                    //     if oldwidth > 800.0 as f64 {
                    //         newwidth = 800.0 as f64;
                    //         newheight = 800.0 as f64 / aspect_ratio.clone();
                    //     };
                    // } else if oldwidth < oldheight {
                    //     println!("portrait");
                    //     if oldheight > 800.0 as f64 {
                    //         newheight = 800.0 as f64;
                    //         newwidth = 800.0 as f64 / aspect_ratio.clone();
                    //     };
                    // } else {
                    //     println!("square");
                    // };
                    // println!(
                    //     "width: {}\nheight: {}\naspect_ratio: {}\nnewwidth: {}\nnewheight: {}\n",
                    //     width, height, aspect_ratio, newwidth, newheight
                    // );

                    // let digest = compute(&oldfilename);
                    // let a = "/media/pi/USB128/Webp/".to_string();
                    // let b = format!("{:?}", digest) + ".jpg";
                    // let newfilename = a + &b;
                    // let _convert =
                    //     convert_image_to_jpg(&oldfilename, &newfilename, newwidth, newheight);
                };
            };
        };
    }
    println!("Total files: {}\n", idx);

    keeper_vec
}

pub fn create_outfile(fname: String) -> String {
    let digest = compute(&fname);
    let a = "/media/pi/USB128/Webp/".to_string();
    let b = format!("{:?}", digest) + ".jpg";
    let newfilename = a + &b;

    newfilename
}

pub fn calc_new_dims(oldwidth: f64, oldheight: f64, aspect: f64) -> (f64, f64) {
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

pub fn mv_small_images(oldwidth: f64, oldheight: f64, fname: String) {
    if oldwidth < 200.0 as f64 || oldheight < 100.0 as f64 {
        let old_fn = fname.clone();
        let fn_parts = old_fn.split("/").collect::<Vec<&str>>();
        let fnam = fn_parts.last().unwrap();
        let new_fn = "/media/pi/USB128/SmallPics/".to_string() + fnam;
        match fs::rename(&fname, &new_fn) {
            Ok(_) => println!("Moved: {}", new_fn),
            Err(e) => println!("Error: {}", e),
        };
    }
}

// pub fn get_aspect_ratio(apath: String) -> Vec<String> {
//     println!("apath: {}", apath);
//     let image = image::open(apath.clone()).expect(&apath);
//     let (width, height) = image.dimensions();
//     let oldwidth = width.clone() as f64;
//     let oldheight = height.clone() as f64;
//     let aspect_ratio = oldwidth / oldheight;
//     let mut av_vec = Vec::new();
//     av_vec.push(apath);
//     av_vec.push(oldwidth.clone().to_string());
//     av_vec.push(oldheight.clone().to_string());
//     av_vec.push(aspect_ratio.clone().to_string());
//     println!(
//         "width: {}\nheight: {}\naspect_ratio: {}\n",
//         oldwidth, oldheight, aspect_ratio);

//     av_vec
// }

pub fn convert_image_to_jpg(file_path: &str, output_path: &str, width: f64, height: f64) {
    let image = image::open(file_path).unwrap();
    let new_image = image.resize(
        width as u32,
        height as u32,
        image::imageops::FilterType::Lanczos3,
    );
    new_image.save(output_path).unwrap();
}
