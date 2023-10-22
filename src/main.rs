// use image::imageops::resize;
// use image::imageops::FilterType::Lanczos3;
use image::GenericImageView;
// use std::sync::mpsc::channel;
// use threadpool::ThreadPool;
use walkdir::WalkDir;

use std::fs;
use std::path::Path;
use std::fs::rename;

pub mod rm_mv_unwanted;
pub mod walk_dirs;


fn main() {
    let _remove_unwanted = rm_mv_unwanted::rm_unwanted_files("/media/pi/0123-4567/Images".to_string());
    let _mv_vid_files = rm_mv_unwanted::mv_vid_files("/media/pi/0123-4567/Images".to_string());

    let extlist = gen_ext_list("/media/pi/0123-4567/Images".to_string());
    println!("extlist: {:?}", extlist);
    let _rm_by_ext = rm_mv_unwanted::rm_by_extension("/media/pi/0123-4567/Images".to_string());

    let new_ext_list = gen_ext_list("/media/pi/0123-4567/Images".to_string());
    println!("new_ext_list: {:?}", new_ext_list);

    let pic_list = walk_dirs::walk_dir("/media/pi/0123-4567/Images".to_string());

    // for pic in pic_list {
    //     let _sanatize = sanitize_filename(Path::new(&pic));
    // }

    for pic in pic_list {
        let _fix_fuckup = fix_fuckup(pic);
    }

    // let _ar = get_aspect_ratio("/media/pi/0123-4567/Images".to_string());

    // let kvec = walk_dirs::walk_dir("/media/pi/0123-4567/Images/".to_string());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for k in kvec {
    //     println!("k: {}", k);
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         get_aspect_ratio(k);
    //         tx.send(()).unwrap();
    //     });
    // }
    // drop(tx);
    // for t in rx.iter() {
    //     let info = t;
    //     println!("info: {:?}", info)
    // }

    // let kvec = walk_dirs::walk_dir(
    //     "/media/pi/58f141b6-81b1-414b-8999-1c86128192c6/Converted/".to_string(),
    // );
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for k in kvec {
    //     println!("k: {}", k);
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         resize_jpg(k);
    //         tx.send(()).unwrap();
    //     });
    // }
    // drop(tx);
    // for t in rx.iter() {
    //     let info = t;
    //     println!("info: {:?}", info)
    // }

    println!("threads complete")
}

fn fix_fuckup(oldfn: String) {
    if oldfn.clone().contains("jpg.jpg") {
        let newfn = oldfn.replace("jpg.jpg", ".jpg");
        rename(oldfn.clone(), newfn.clone()).unwrap();
        println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
    } else if oldfn.clone().contains("bmp.bmp") {
        let newfn = oldfn.replace("bmp.bmp", ".bmp");
        rename(oldfn.clone(), newfn.clone()).unwrap();
        println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
    } else if oldfn.clone().contains("gif.gif") {
        let newfn = oldfn.replace("gif.gif", ".gif");
        rename(oldfn.clone(), newfn.clone()).unwrap();
        println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
    } else if oldfn.clone().contains("png.png") {
        let newfn = oldfn.replace("png.png", ".png");
        rename(oldfn.clone(), newfn.clone()).unwrap();
        println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
    } else if oldfn.clone().contains("jpeg.jpeg") {
        let newfn = oldfn.replace("jpeg.jpeg", ".jpg");
        rename(oldfn.clone(), newfn.clone()).unwrap();
        println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
    } else {
        println!("{}", oldfn)
    }





}

// fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
//     let filename = path.file_name().unwrap().to_str().unwrap();
//     let extension = path.extension().unwrap().to_str().unwrap();

//     let mut new_filename = String::new();

//     for c in filename.chars() {
//         if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
//             new_filename.push(c);
//         }
//     }

//     let new_filename = new_filename.to_lowercase();
//     let new_filename = format!("{}.{}", new_filename, extension.to_lowercase());

//     let new_path = path.parent().unwrap().join(&new_filename);

//     println!("new_path: \n\t{:?}\n\t{:?}\n", path, new_path);

//     rename(path, &new_path)?;

//     Ok(new_filename)
// }

fn gen_ext_list(apath: String) -> Vec<String> {
    let mut ext_list: Vec<String> = Vec::new();
    for e in WalkDir::new(apath)
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

fn mv_to_banner_folder(apath: String) {
    let fparts = apath.split("/").collect::<Vec<&str>>();
    let filename = fparts.last().unwrap().replace(" ", "_");
    let addr = "/media/pi/0123-4567/Banners/".to_string() + &filename;
    println!("addr: {}\n apath: {}\n", addr, apath);
    match fs::rename(&apath, &addr) {
        Ok(_) => println!("Moved: {}", addr),
        Err(e) => println!("Error: {}", e),
    };
}



fn filter_by_aspect_ratio(apath: String) -> bool {
    let keeplist = [
        "jpg".to_string(),
        "JPG".to_string(),
        "bmp".to_string(),
        "BMP".to_string(),
        "gif".to_string(),
        "png".to_string(),
        "tif".to_string(),
        "jpeg".to_string(),
        "PNG".to_string(),
        "GIF".to_string(),
    ];

    // let mut listvec = Vec::new();

    let fname = Path::new(&apath);
    let filename = fname.to_string_lossy().to_string();
    if let Some(extension) = fname.extension() {
        let ext = extension.to_owned().to_str().unwrap().to_string();
        // let mut av_vec = Vec::new();
        if keeplist.contains(&ext) {
            let image = image::open(filename.clone()).expect(&filename);
            let (width, height) = image.dimensions();
            let oldwidth = width.clone() as f64;
            let oldheight = height.clone() as f64;
            let aspect_ratio = oldwidth / oldheight;
            if oldwidth > oldheight {
                println!("landscape");

            } else if oldwidth < oldheight {
                print!("portrait")

            } else if oldwidth == oldheight {
                println!("square")

            } else {

            }
            // av_vec.push(oldwidth.clone());
            // av_vec.push(oldheight.clone());
            // av_vec.push(aspect_ratio.clone());
            // if aspect_ratio > 2.0 {
            //     let _mv_banner_image = mv_to_banner_folder(filename.clone());
            //     println!("Filename: {}\n aspect_ratio: {}\n", filename, aspect_ratio);
            // } else {
            //     let new_dims = walk_dirs::calc_new_dims(oldwidth, oldheight, aspect_ratio);
            //     let newwidth = new_dims.0;
            //     let newheight = new_dims.1;
            //     let output_file = walk_dirs::create_outfile(filename.clone());
            //     let resized = resize(&image, newwidth as u32, newheight as u32, Lanczos3);
            //     resized.save(output_file.clone()).unwrap();
            //     println!(
            //         "width: {}\nheight: {}\naspect_ratio: {}\n",
            //         width, height, aspect_ratio
            //     );
            // }
        };
        // listvec.push(av_vec.clone());
    };

    // listvec
    true
}

// fn resize_jpg(ajpg: String) {
//     let image = image::open(ajpg.clone()).expect(&ajpg);
//     let (width, height) = image.dimensions();
//     let oldwidth = width.clone() as f64;
//     let oldheight = height.clone() as f64;
//     let aspect_ratio = oldwidth / oldheight;
//     let new_dims = walk_dirs::calc_new_dims(oldwidth, oldheight, aspect_ratio);
//     let newwidth = new_dims.0;
//     let newheight = new_dims.1;
//     let output_file = walk_dirs::create_outfile(ajpg.clone());
//     let resized = resize(&image, newwidth as u32, newheight as u32, Lanczos3);
//     resized.save(output_file.clone()).unwrap();
//     println!(
//         "width: {}\nheight: {}\naspect_ratio: {}\n",
//         width, height, aspect_ratio
//     );
// }
