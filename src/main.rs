// use image::imageops::resize;
// use image::imageops::FilterType::Lanczos3;
// use image::GenericImageView;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
// use walkdir::WalkDir;

// use std::fs;
use std::fs::rename;
use std::path::Path;

// use crate::factory::convert_image_to_jpg;

pub mod factory;
pub mod rm_mv_unwanted;
pub mod walk_dirs;
pub mod dedup;

fn main() {
    let _remove_unwanted =
        rm_mv_unwanted::rm_unwanted_files("/media/pi/0123-4567/Images".to_string());
    let _mv_vid_files = rm_mv_unwanted::mv_vid_files("/media/pi/0123-4567/Images".to_string());

    let extlist = factory::gen_ext_list("/media/pi/0123-4567/Images".to_string());
    println!("extlist: {:?}", extlist);
    let _rm_by_ext = rm_mv_unwanted::rm_by_extension("/media/pi/0123-4567/Images".to_string());

    let new_ext_list = factory::gen_ext_list("/media/pi/0123-4567/Images".to_string());
    println!("new_ext_list: {:?}", new_ext_list);

    let pic_list = walk_dirs::walk_dir("/media/pi/0123-4567/Images".to_string());
    // for pic in pic_list.clone() {
    //     let _sanatize = sanitize_filename(Path::new(&pic));
    // }

    // let pic_list2 = walk_dirs::walk_dir("/media/pi/0123-4567/Images".to_string());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for pic in pic_list2.clone() {
    //     println!("Pic {}", pic);
    //     if !pic.contains(".jpg") {
    //         let tx = tx.clone();
    //         pool.execute(move || {
    //             factory::convert_image_to_jpg(pic.clone());
    //             tx.send(()).unwrap();
    //         });
    //     };
    // }
    // drop(tx);
    // for t in rx.iter() {
    //     let info = t;
    //     println!("info: {:?}", info)
    // }

    // let all_jpgs = walk_dirs::walk_dir("/media/pi/0123-4567/Images".to_string());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for jpg in all_jpgs {
    //     println!("jpg {}", jpg);
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         mv_jpgs(jpg.clone());
    //         tx.send(()).unwrap();
    //     });
    // }
    // drop(tx);
    // for t in rx.iter() {
    //     let info = t;
    //     println!("info: {:?}", info)
    // }

    for pic in pic_list {
        let _dedup = dedup::calc_hash(pic);
    };

    println!("threads complete")
}

fn mv_jpgs(fname: String) -> String {
    let pf = factory::Factory {
        path: fname.clone(),
    };
    let outfile = pf.create_outfile();
    if fname.contains(".jpg") {
        let image = image::open(fname.clone()).expect(&fname);
        image.save(outfile.clone()).unwrap()
    }
    let foo = format!("Moved {:?}\n to {:?}", fname, outfile.clone());

    foo
}

// fn mv_to_banner_folder(apath: String) {
//     let fparts = apath.split("/").collect::<Vec<&str>>();
//     let filename = fparts.last().unwrap().replace(" ", "_");
//     let addr = "/media/pi/0123-4567/Banners/".to_string() + &filename;
//     println!("addr: {}\n apath: {}\n", addr, apath);
//     match fs::rename(&apath, &addr) {
//         Ok(_) => println!("Moved: {}", addr),
//         Err(e) => println!("Error: {}", e),
//     };
// }

// fn filter_by_aspect_ratio(apath: String) -> bool {
//     let keeplist = [
//         "jpg".to_string(),
//         "JPG".to_string(),
//         "bmp".to_string(),
//         "BMP".to_string(),
//         "gif".to_string(),
//         "png".to_string(),
//         "tif".to_string(),
//         "jpeg".to_string(),
//         "PNG".to_string(),
//         "GIF".to_string(),
//     ];

//     // let mut listvec = Vec::new();

//     let fname = Path::new(&apath);
//     let filename = fname.to_string_lossy().to_string();
//     if let Some(extension) = fname.extension() {
//         let ext = extension.to_owned().to_str().unwrap().to_string();
//         // let mut av_vec = Vec::new();
//         if keeplist.contains(&ext) {
//             let image = image::open(filename.clone()).expect(&filename);
//             let (width, height) = image.dimensions();
//             let oldwidth = width.clone() as f64;
//             let oldheight = height.clone() as f64;
//             let aspect_ratio = oldwidth / oldheight;
//             if oldwidth > oldheight {
//                 println!("landscape");

//             } else if oldwidth < oldheight {
//                 print!("portrait")

//             } else if oldwidth == oldheight {
//                 println!("square")

//             } else {

//             }
//             // av_vec.push(oldwidth.clone());
//             // av_vec.push(oldheight.clone());
//             // av_vec.push(aspect_ratio.clone());
//             // if aspect_ratio > 2.0 {
//             //     let _mv_banner_image = mv_to_banner_folder(filename.clone());
//             //     println!("Filename: {}\n aspect_ratio: {}\n", filename, aspect_ratio);
//             // } else {
//             //     let new_dims = walk_dirs::calc_new_dims(oldwidth, oldheight, aspect_ratio);
//             //     let newwidth = new_dims.0;
//             //     let newheight = new_dims.1;
//             //     let output_file = walk_dirs::create_outfile(filename.clone());
//             //     let resized = resize(&image, newwidth as u32, newheight as u32, Lanczos3);
//             //     resized.save(output_file.clone()).unwrap();
//             //     println!(
//             //         "width: {}\nheight: {}\naspect_ratio: {}\n",
//             //         width, height, aspect_ratio
//             //     );
//             // }
//         };
//         // listvec.push(av_vec.clone());
//     };

//     // listvec
//     true
// }

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

fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    // let extension = path.extension().unwrap().to_str().unwrap();

    let mut new_filename = String::new();

    for c in filename.chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
            new_filename.push(c);
        }
    }

    let new_filename = new_filename.to_lowercase();

    let new_path = path.parent().unwrap().join(&new_filename);

    println!("new_path: \n\t{:?}\n\t{:?}\n", path, new_path);

    rename(path, &new_path)?;

    Ok(new_filename)
}
