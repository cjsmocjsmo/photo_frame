extern crate img_hash;
use img_hash::HasherConfig;
use img_hash::ImageHash;

#[derive(Clone, Debug)]
pub struct ImgHashStruct {
    pub path: String,
    pub hash: ImageHash,
}
pub fn calc_hash(apath: String) -> ImgHashStruct {
    // Create a hasher config.
    let hasher_config = HasherConfig::new().to_hasher();

    // Read the image file.
    let image = image::open(apath.clone()).unwrap();

    // Calculate the pHash of the image.
    let hashed = hasher_config.hash_image(&image);

    let imghash = ImgHashStruct {
        path: apath.clone(),
        hash: hashed,
    };

    // Print the pHash.
    // println!("{:#?}", imghash);

    imghash
}



// pub fn compare_hashes(hash1: &str, hash2: &str) -> f64 {
    // Create a hasher config.
    // let hasher_config = HasherConfig::new()
    //     .hash_size(8, 8)
    //     .hash_alg(HashAlg::DoubleGradient)
    //     .to_hasher();



    // // Parse the hashes.
    // let parsed_hash1 = hash1.parse::<u64>().unwrap();
    // let parsed_hash2 = hash2.parse::<u64>().unwrap();

    // Calculate the distance between the hashes.
//     let distance = hash1.dist(&hash2);

//     distance

// }