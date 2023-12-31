use std::fs;
use std::fs::rename;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Instant;
use threadpool::ThreadPool;
// use image::GenericImageView;

pub mod dedup;
pub mod factory;
pub mod rm_mv_unwanted;
pub mod walk_dirs;
pub mod zip;

fn main() {
    let start = Instant::now();

    let _prepenv = prep_env();
    let url = "/media/pipi/0123-4567/Images".to_string();
    let _mv_vid_files = rm_mv_unwanted::mv_vid_files(url.clone());

    let _mv_zip_files = rm_mv_unwanted::mv_zip_files(url.clone());
    let _process_zip_files = zip::process_zip_files();
    let _process_gz_files = zip::process_gz_files();
    let _process_bz2_files = zip::process_bz2_files();

    let _mv_zip_files = rm_mv_unwanted::mv_zip_files(url.clone());
    let _process_zip_files2 = zip::process_zip_files();
    let _process_gz_files2 = zip::process_gz_files();
    let _process_bz2_files2 = zip::process_bz2_files();

    let _mv_zip_files = rm_mv_unwanted::mv_zip_files(url.clone());
    let _process_zip_files3 = zip::process_zip_files();
    let _process_gz_files3 = zip::process_gz_files();
    let _process_bz2_files3 = zip::process_bz2_files();

    let _mv_vid_files = rm_mv_unwanted::mv_vid_files(url.clone());

    let rm_unwanted_count = rm_mv_unwanted::rm_unwanted_files(url.clone());

    let extlist = factory::gen_ext_list(url.clone());
    println!("extlist: {:?}", extlist);
    let rm_by_ext_count = rm_mv_unwanted::rm_by_extension(url.clone());

    let new_ext_list = factory::gen_ext_list(url.clone());
    println!("new_ext_list: {:?}", new_ext_list);

    let pic_list = walk_dirs::walk_dir(url.clone());
    for pic in pic_list.clone() {
        let _sanatize = sanitize_filename(Path::new(&pic));
    }

    let pic_list2 = walk_dirs::walk_dir(url.clone());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for pic in pic_list2.clone() {
        let tx = tx.clone();
        pool.execute(move || {
            factory::convert_image_to_jpg(pic.clone());
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    for t in rx.iter() {
        let _info = t;
        // println!("info: {:?}", info)
    }

    let url2 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/".to_string();

    let pic_list3 = walk_dirs::walk_dir(url2.clone());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for pic in pic_list3.clone() {
        let tx = tx.clone();
        pool.execute(move || {
            dedup::calc_hash_test(pic.clone());
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    for t in rx.iter() {
        let _info = t;
        // println!("info: {:?}", info)
    }

    let pic_list2 = walk_dirs::walk_dir(url2.clone());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for jpg in pic_list2 {
        println!("jpg {}", jpg);
        let tx = tx.clone();
        pool.execute(move || {
            let dd = dedup::calc_hash(jpg.clone());
            tx.send(dd).unwrap();
        });
    }
    drop(tx);
    let mut img_hash_list = Vec::new();
    for t in rx.iter() {
        let info = t;
        img_hash_list.push(info.clone());
        println!("info: {:?}", info.clone());
    }

    // println!("img_hash_list: {:?}", img_hash_list.clone().len());
    let file_list = walk_dirs::walk_dir(url2.clone());

    let mut dup_results = Vec::new();
    for jpg in file_list {
        println!("jpg: {}", jpg);
        let image_hash_list2 = img_hash_list.clone();
        let dd = dedup::compare_hashes(jpg.clone(), image_hash_list2.clone());
        dup_results.push(dd.clone());
    }

    let url3 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove".to_string();
    let url4 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master/".to_string();

    let json_list = walk_dirs::walk_dir(url3.clone());
    for json_file in json_list.clone() {
        let json_content = fs::read_to_string(json_file.clone()).expect("Unable to read file");
        let dups_entry: dedup::TransDupsEntry = serde_json::from_str(&json_content).unwrap();
        // println!("dups_entry: {:#?}", dups_entry);
        let keep_file = dups_entry.filename.clone();
        println!("keep_file: {:#?}", keep_file);
        let keep_file_exists = Path::new(&keep_file).exists();
        if keep_file_exists {
            let keep_file_parts = keep_file.split("/").collect::<Vec<&str>>();
            let fname = keep_file_parts.last().unwrap().to_string();
            let newfilename = url4.clone() + &fname;
            println!("newfilename: {:#?}", newfilename);
            fs::rename(keep_file.clone(), newfilename.clone()).expect("Unable to rename file");
            let dups = dups_entry.duplicates.clone();
            for dup in dups {
                let dup_url = url2.clone() + &dup.strdups.to_string();
                let dup_url_exists = Path::new(&dup_url).exists();
                if dup_url_exists {
                    let _rm_dup = fs::remove_file(dup_url.clone()).expect("Unable to delete file");
                    println!("Deleted: \n\t{}", dup_url.clone());
                } else {
                    println!("File does not exist: \n\t{}", dup_url.clone());
                }
            }
        }
    }
    // let url5 = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Fixed_Needs_Renaming/".to_string();
    // let f3 = factory::Factory { path: url4.clone() };
    // let master_count = f3.dir_count();
    // println!("master_count: {}", master_count);



    // let _fixedfiles = factory::rename_fixed_files(url5);



    // let pic_list_master = walk_dirs::walk_dir("/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master/".to_string());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for jpg in pic_list_master {
    //     println!("jpg {}", jpg);
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         let filter_by_aspect_ratio = filter_by_aspect_ratio(jpg.clone());
    //         tx.send(filter_by_aspect_ratio).unwrap();
    //     });
    // }
    // drop(tx);

    // for t in rx.iter() {
    //     let info = t;
    //     println!("aspect-ratio: {:?}", info.clone());
    // }





    let f1 = factory::Factory { path: url.clone() };
    let original_count = f1.dir_count();
    println!("original_count: {}", original_count);

    let f2 = factory::Factory { path: url2.clone() };
    let converted_count = f2.dir_count();
    println!("converted_count: {}", converted_count);

    let f3 = factory::Factory { path: url4.clone() };
    let master_count = f3.dir_count();
    println!("master_count: {}", master_count);

    let total_rm_count = rm_unwanted_count + rm_by_ext_count;
    println!("Total files removed: {}", total_rm_count);

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
    let b_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BadImages/";
    let master_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master/";
    let gz0 = "/media/pipi/0123-4567/AV/";
    let gz1_path = "/media/pipi/0123-4567/GZ1/";
    let zip_path = "/media/pipi/0123-4567/ZIP/";
    let bz2_path = "/media/pipi/0123-4567/BZ2/";
    let bad_path = "/media/pipi/0123-4567/BadImages/";
    let gz1_unzip_path = "/media/pipi/0123-4567/Images/GZ1_Unzip/";
    let gz2_unzip_path = "/media/pipi/0123-4567/Images/GZ2_Unzip/";
    let zip_unzip_path = "/media/pipi/0123-4567/Images/ZIP_Unzip/";
    let bz2_unzip_path = "/media/pipi/0123-4567/Images/BZ2_Unzip/";

    let mut zip_list = Vec::new();
    zip_list.push(converted_path);
    zip_list.push(toremove_path);
    zip_list.push(gz1_path);
    zip_list.push(zip_path);
    zip_list.push(bz2_path);
    zip_list.push(gz1_unzip_path);
    zip_list.push(gz2_unzip_path);
    zip_list.push(zip_unzip_path);
    zip_list.push(bz2_unzip_path);
    zip_list.push(bad_path);
    zip_list.push(b_path);
    zip_list.push(gz0);
    zip_list.push(master_path);

    let _: Vec<_> = zip_list.iter().map(|x| pf_create_dir(x)).collect();
}

pub fn pf_create_dir(apath: &str) {
    let save_dir = Path::new(apath);
    if fs::metadata(save_dir).is_ok() {
        fs::remove_dir_all(save_dir).expect("Unable to remove directory");
        fs::create_dir_all(save_dir).expect("Unable to create directory");
    } else {
        fs::create_dir_all(save_dir).expect("Unable to create directory");
    }
}

fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
    let filename = path.file_name().unwrap().to_str().unwrap();
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

pub fn mv_to_banner_folder(apath: String) {
    let fparts = apath.split("/").collect::<Vec<&str>>();
    let filename = fparts.last().unwrap().replace(" ", "_");
    let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Banners/".to_string() + &filename;
    println!("addr: {}\n apath: {}\n", addr, apath);
    match fs::rename(&apath, &addr) {
        Ok(_) => println!("Moved: {}", addr),
        Err(e) => println!("Error: {}", e),
    };
}

pub fn mv_to_square_folder(apath: String) {
    let fparts = apath.split("/").collect::<Vec<&str>>();
    let filename = fparts.last().unwrap().replace(" ", "_");
    let addr = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Square/".to_string() + &filename;
    println!("addr: {}\n apath: {}\n", addr, apath);
    match fs::rename(&apath, &addr) {
        Ok(_) => println!("Moved: {}", addr),
        Err(e) => println!("Error: {}", e),
    };
}

// fn filter_by_aspect_ratio(apath: String) -> f64{
//     let image = image::open(apath.clone()).expect(&apath);
//     let (width, height) = image.dimensions();
//     let oldwidth = width.clone() as f64;
//     let oldheight = height.clone() as f64;
//     let aspect_ratio = oldwidth / oldheight;
//     // if oldwidth > oldheight {
//     //     println!("landscape");
//     // } else if oldwidth < oldheight {
//     //     print!("portrait")
//     // } else {
//     //     println!("square")
//     // };

//     if aspect_ratio > 1.9 {
//         let _mv_banner_image = mv_to_banner_folder(apath.clone());
//         println!("Filename: {}\n aspect_ratio: {}\n", apath, aspect_ratio);
//     };
//     if aspect_ratio == 1.0 {
//         let _mv_square_image = mv_to_square_folder(apath.clone());
//         println!("Filename: {}\n aspect_ratio: {}\n", apath, aspect_ratio);
//     };



//     aspect_ratio
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
