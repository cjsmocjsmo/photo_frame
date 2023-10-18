use image::GenericImageView;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
pub mod walk_dirs;


fn main() {
    let kvec = walk_dirs::walk_dir("/media/pi/USB128/Images/WendyPics".to_string());
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for k in kvec {
        let tx = tx.clone();
        pool.execute(move || {
            find(k);
            tx.send(()).unwrap();
        });
    }
    drop(tx);
    for t in rx.iter() {
        let _info = t;
    }

    println!("threads complete")
}

fn find(k: String) {

        let dims = get_aspect_ratio(k.clone());
        let width: f64 = dims[0];
        let height: f64 = dims[1];
        let aspect_ratio: f64 = dims[2];
        let _mvsmimg = walk_dirs::mv_small_images(width as f64, height, k.clone());
        let new_dims = walk_dirs::calc_new_dims(width, height, aspect_ratio);
        let newwidth = new_dims.0;
        let newheight = new_dims.1;

        let out_file = walk_dirs::create_outfile(k.clone());
        let _pimage = walk_dirs::convert_image_to_jpg(&k, &out_file, newwidth, newheight);
        println!(
            "width: {}\nheight: {}\naspect_ratio: {}\n",
            width, height, aspect_ratio
        );

}

fn get_aspect_ratio(apath: String) -> Vec<f64> {
    let image = image::open(apath.clone()).expect(&apath);
    let (width, height) = image.dimensions();
    let oldwidth = width.clone() as f64;
    let oldheight = height.clone() as f64;
    let aspect_ratio = oldwidth / oldheight;
    let mut av_vec = Vec::new();
    // av_vec.push(apath);
    av_vec.push(oldwidth.clone());
    av_vec.push(oldheight.clone());
    av_vec.push(aspect_ratio.clone());

    av_vec
}
