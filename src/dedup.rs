use cv::imgproc;
use std::collections::HashSet;
use std::fs::{read_dir, metadata};
use std::path::Path;
// use opencv::prelude::*;

fn main() {
    let mut image_set = HashSet::new();

    for path in read_dir("/media/pi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/").unwrap() {
        let path = path.unwrap().path();

        if metadata(&path).unwrap().is_file() && path.extension().unwrap() == "jpg" {
            let image = imgproc::imread(&path.to_string(), imgproc::IMREAD_COLOR).unwrap();

            // Calculate the features of the image.
            let features = imgproc::SIFT_create().detect_and_compute(&image, None).unwrap();

            image_set.insert(features);
            println!("path: {}", features);
        }
    }

    for features in image_set.iter() {
        if image_set.contains(&features) {
            println!("Found duplicate image: {}", path.to_string());
        }
    }
}

// fn fix_fuckup(oldfn: String) {
//     if oldfn.clone().contains("jpg.jpg") {
//         let newfn = oldfn.replace("jpg.jpg", ".jpg");
//         rename(oldfn.clone(), newfn.clone()).unwrap();
//         println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
//     } else if oldfn.clone().contains("bmp.bmp") {
//         let newfn = oldfn.replace("bmp.bmp", ".bmp");
//         rename(oldfn.clone(), newfn.clone()).unwrap();
//         println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
//     } else if oldfn.clone().contains("gif.gif") {
//         let newfn = oldfn.replace("gif.gif", ".gif");
//         rename(oldfn.clone(), newfn.clone()).unwrap();
//         println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
//     } else if oldfn.clone().contains("png.png") {
//         let newfn = oldfn.replace("png.png", ".png");
//         rename(oldfn.clone(), newfn.clone()).unwrap();
//         println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
//     } else if oldfn.clone().contains("jpeg.jpeg") {
//         let newfn = oldfn.replace("jpeg.jpeg", ".jpg");
//         rename(oldfn.clone(), newfn.clone()).unwrap();
//         println!("oldfn: {}\n newfn: {}\n", oldfn.clone(), newfn.clone())
//     } else {
//         println!("{}", oldfn)
//     }
// }