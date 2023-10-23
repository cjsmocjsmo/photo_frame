extern crate img_hash;
use img_hash::{HashAlg, HasherConfig};

pub fn calc_hash(apath: String) {
    // Create a hasher config.
    let hasher_config = HasherConfig::new()
        .hash_size(8, 8)
        .hash_alg(HashAlg::DoubleGradient)
        .to_hasher();

    // Read the image file.
    let image = image::open(apath.clone()).unwrap();

    // Calculate the pHash of the image.
    let hash = hasher_config.hash_image(&image);

    // Print the pHash.
    println!("{:?}", hash);
}
