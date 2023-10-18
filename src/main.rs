// use image::GenericImageView;
// use std::sync::mpsc::channel;
// use threadpool::ThreadPool;
use walkdir::WalkDir;
pub mod walk_dirs;
pub mod rm_mv_unwanted;
// use std::fs;
// use std::path::Path;


fn main() {
    let _remove_unwanted = rm_mv_unwanted::rm_unwanted_files("/media/pi/USB128/Images".to_string());
    // let _mv_vid_files = rm_mv_unwanted::mv_vid_files("/media/pi/USB128/Images".to_string());


    // let _extlist = gen_ext_list("/media/pi/USB128/Images".to_string());
    // let _rm_by_ext = rm_mv_unwanted::rm_by_extension("/media/pi/USB128/Images".to_string());

    // let new_ext_list = gen_ext_list("/media/pi/USB128/Images".to_string());
    // println!("new_ext_list: {:?}", new_ext_list);

    // let kvec = walk_dirs::walk_dir("/media/pi/USB128/Images/WendyPics".to_string());
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();
    // for k in kvec {
    //     let tx = tx.clone();
    //     pool.execute(move || {
    //         find(k);
    //         tx.send(()).unwrap();
    //     });
    // }
    // drop(tx);
    // for t in rx.iter() {
    //     let _info = t;
    // }

    // println!("threads complete")
}

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

// fn find(k: String) {

//         let dims = get_aspect_ratio(k.clone());
//         let width: f64 = dims[0];
//         let height: f64 = dims[1];
//         let aspect_ratio: f64 = dims[2];
//         let _mvsmimg = walk_dirs::mv_small_images(width as f64, height, k.clone());
//         let new_dims = walk_dirs::calc_new_dims(width, height, aspect_ratio);
//         let newwidth = new_dims.0;
//         let newheight = new_dims.1;

//         let out_file = walk_dirs::create_outfile(k.clone());
//         let _pimage = walk_dirs::convert_image_to_jpg(&k, &out_file, newwidth, newheight);
//         println!(
//             "width: {}\nheight: {}\naspect_ratio: {}\n",
//             width, height, aspect_ratio
//         );

// }

// fn get_aspect_ratio(apath: String) -> Vec<f64> {
//     let image = image::open(apath.clone()).expect(&apath);
//     let (width, height) = image.dimensions();
//     let oldwidth = width.clone() as f64;
//     let oldheight = height.clone() as f64;
//     let aspect_ratio = oldwidth / oldheight;
//     let mut av_vec = Vec::new();
//     // av_vec.push(apath);
//     av_vec.push(oldwidth.clone());
//     av_vec.push(oldheight.clone());
//     av_vec.push(aspect_ratio.clone());

//     av_vec
// }
