use image::GenericImageView;
// use std::sync::mpsc::channel;
// use threadpool::ThreadPool;
use walkdir::WalkDir;
pub mod rm_mv_unwanted;
pub mod walk_dirs;
// use std::fs;
// use std::path::Path;

fn main() {
    let _remove_unwanted = rm_mv_unwanted::rm_unwanted_files("/media/pi/USB128/Images".to_string());
    let _mv_vid_files = rm_mv_unwanted::mv_vid_files("/media/pi/USB128/Images".to_string());

    let extlist = gen_ext_list("/media/pi/USB128/Images".to_string());
    println!("extlist: {:?}", extlist);
    let _rm_by_ext = rm_mv_unwanted::rm_by_extension("/media/pi/USB128/Images".to_string());

    let new_ext_list = gen_ext_list("/media/pi/USB128/Images".to_string());
    println!("new_ext_list: {:?}", new_ext_list);

    let ar = get_aspect_ratio("/media/pi/USB128/Images".to_string());
    for a in ar{
        if a[2] > 1.9 {
            println!("a: {:?}", a);
        };
    }
    // println!("ar: {:?}", ar);

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

fn get_aspect_ratio(apath: String) -> Vec<Vec<f64>> {
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

    let mut listvec = Vec::new();
    let mut count = 0;
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            count += 1;
            println!("count: {}", count);
            let fname = e.path();
            let filename = e.path().to_string_lossy().to_string();
            if let Some(extension) = fname.extension() {
                let ext = extension.to_owned().to_str().unwrap().to_string();
                let av_vec = Vec::new();
                if keeplist.contains(&ext) {
                    let image = image::open(filename.clone()).expect(&filename);
                    let (width, height) = image.dimensions();
                    let oldwidth = width.clone() as f64;
                    let oldheight = height.clone() as f64;
                    let aspect_ratio = oldwidth / oldheight;
                    let mut av_vec = Vec::new();
                    // av_vec.push(fname);
                    av_vec.push(oldwidth.clone());
                    av_vec.push(oldheight.clone());
                    av_vec.push(aspect_ratio.clone());
                };
                listvec.push(av_vec);
                println!("{:#?}", listvec)
            };
        };
    };

    listvec
}
