use cv::imgproc;
use std::collections::HashSet;
use std::fs::{read_dir, metadata};
use std::path::Path;

fn main() {
    let mut image_set = HashSet::new();

    for path in read_dir("/path/to/photo/gallery").unwrap() {
        let path = path.unwrap().path();

        if metadata(&path).unwrap().is_file() && path.extension().unwrap() == "jpg" {
            let image = imgproc::imread(&path.to_string(), imgproc::IMREAD_COLOR).unwrap();

            // Calculate the features of the image.
            let features = imgproc::SIFT_create().detect_and_compute(&image, None).unwrap();

            image_set.insert(features);
        }
    }

    for features in image_set.iter() {
        if image_set.contains(&features) {
            println!("Found duplicate image: {}", path.to_string());
        }
    }
}