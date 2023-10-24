extern crate img_hash;
use img_hash::{HashAlg, HasherConfig};


pub fn calc_hash(apath: String) -> Vec<String> {
    // Create a hasher config.
    let hasher_config = HasherConfig::new()
        // .hash_size(8, 8)
        // .hash_alg(HashAlg::DoubleGradient)
        .to_hasher();

    // Read the image file.
    let image = image::open(apath.clone()).unwrap();

    // Calculate the pHash of the image.
    let hash = hasher_config.hash_image(&image);

    let mut hashvec = Vec::new();
    hashvec.push(apath.clone());
    let hash_string = format!("{:?}", hash);
    hashvec.push(hash_string);

    // Print the pHash.
    println!("{:?}", hashvec);

    hashvec
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