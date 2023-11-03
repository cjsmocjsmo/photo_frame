use std::fs;
// use std::fs::rename;
use std::path::Path;
// use std::sync::mpsc::channel;
use std::time::Instant;
// use subprocess::Exec;
// use threadpool::ThreadPool;

pub mod dedup;
pub mod factory;
pub mod rm_mv_unwanted;
pub mod walk_dirs;
pub mod zip;

fn main() {
    let start = Instant::now();

    // let _prepenv = prep_env();
    let url = "/media/pipi/0123-4567/Images".to_string();

    // let url2 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted".to_string();

    // let _rm_unwanted = rm_mv_unwanted::rm_unwanted_files(url.clone());
    // let _mv_vid_files = rm_mv_unwanted::mv_vid_files(url.clone());
    // let _mv_zip_files = rm_mv_unwanted::mv_zip_files(url.clone());

    let zip_url = "/media/pipi/0123-4567/ZIP".to_string();
    let _process_zip_files = zip::process_zip_files(zip_url.clone());

    // let gz_url = "/media/pipi/0123-4567/GZ".to_string();
    // let _process_gz_files = zip::process_gz_files(url.clone());

    // let bz2_url = "/media/pipi/0123-4567/BZ2".to_string();
    // let _process_bz2_files = zip::process_bz2_files(url.clone());

    // let extlist = factory::gen_ext_list(url.clone());
    // println!("extlist: {:?}", extlist);
    // let _rm_by_ext = rm_mv_unwanted::rm_by_extension(url.clone());

    // let new_ext_list = factory::gen_ext_list(url.clone());
    // println!("new_ext_list: {:?}", new_ext_list);

    // let pic_list = walk_dirs::walk_dir(url.clone());
    // for pic in pic_list.clone() {
    //     let _sanatize = sanitize_filename(Path::new(&pic));
    // }

    // let pic_list2 = walk_dirs::walk_dir(url.clone());
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

    // let pic_list2 = walk_dirs::walk_dir(url2.clone());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for jpg in pic_list2 {
    //     println!("jpg {}", jpg);
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         let dd = dedup::calc_hash(jpg.clone());
    //         tx.send(dd).unwrap();
    //     });
    // }
    // drop(tx);
    // let mut img_hash_list = Vec::new();
    // for t in rx.iter() {
    //     let info = t;
    //     img_hash_list.push(info.clone());
    //     println!("info: {:?}", info.clone());
    // }

    // println!("img_hash_list: {:?}", img_hash_list.clone().len());
    // let file_list = walk_dirs::walk_dir(url2.clone());

    // let mut dup_results = Vec::new();
    // for jpg in file_list {
    //     let image_hash_list2 = img_hash_list.clone();
    //     let dd = dedup::compare_hashes(jpg.clone(), image_hash_list2.clone());
    //     dup_results.push(dd.clone());
    // }

    // println!(
    //     "dups_result count: {:#?}\n threads complete",
    //     dup_results.clone()
    // );
    // let end = Instant::now();
    let elapsed = start.elapsed().as_secs_f64();
    println!("Execution time: {}", elapsed)
}

fn prep_env() {
    let av_path = "/media/pipi/0123-4567/AV/";
    let av_save_dir = Path::new(av_path);
    if !fs::metadata(av_save_dir).is_ok() {
        fs::create_dir(av_save_dir).expect("Unable to create AV directory");
    }

    let converted_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
    let toremove_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/";
    let g1_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/G1/";
    let g2_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/G2/";
    let zpath = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ZIP/";
    let bpath = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BZ2/";
    let gz1_path = "/media/pipi/0123-4567/GZ1/";
    let gz2_path = "/media/pipi/0123-4567/GZ2/";
    let zip_path = "/media/pipi/0123-4567/ZIP/";
    let bz2_path = "/media/pipi/0123-4567/BZ2/";

    let mut path_list = Vec::new();
    path_list.push(converted_path);
    path_list.push(toremove_path);
    path_list.push(g1_path);
    path_list.push(g2_path);
    path_list.push(zpath);
    path_list.push(bpath);
    path_list.push(gz1_path);
    path_list.push(gz2_path);
    path_list.push(zip_path);
    path_list.push(bz2_path);

    for p in path_list {
        let _cd = create_dir(&p);
    }
}

fn create_dir(apath: &str) {
    let save_dir = Path::new(apath);
    if fs::metadata(save_dir).is_ok() {
        fs::remove_dir_all(save_dir).expect("Unable to remove directory");
        fs::create_dir(save_dir).expect("Unable to create directory");
    } else {
        fs::create_dir(save_dir).expect("Unable to create directory");
    }
}

// fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
//     let filename = path.file_name().unwrap().to_str().unwrap();
//     let mut new_filename = String::new();
//     for c in filename.chars() {
//         if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
//             new_filename.push(c);
//         }
//     }
//     let new_filename = new_filename.to_lowercase();
//     let new_path = path.parent().unwrap().join(&new_filename);
//     println!("new_path: \n\t{:?}\n\t{:?}\n", path, new_path);
//     rename(path, &new_path)?;

//     Ok(new_filename)
// }

// fn mv_to_banner_folder(apath: String) {
//     let fparts = apath.split("/").collect::<Vec<&str>>();
//     let filename = fparts.last().unwrap().replace(" ", "_");
//     let addr = "/media/pipi/0123-4567/Banners/".to_string() + &filename;
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
