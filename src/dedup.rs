extern crate img_hash;
use img_hash::HasherConfig;
use img_hash::ImageHash;

#[derive(Clone, Debug)]
pub struct ImgHashStruct {
    pub img_path: String,
    pub hash: ImageHash,
}
pub fn calc_hash(apath: String) -> ImgHashStruct {
    // Create a hasher config.
    let hasher_config = HasherConfig::new().to_hasher();

    // Read the image file.
    let image = image::open(apath.clone()).expect(&apath);

    // Calculate the pHash of the image.
    let hashed = hasher_config.hash_image(&image);

    let imghash = ImgHashStruct {
        img_path: apath.clone(),
        hash: hashed,
    };

    imghash
}


#[derive(Clone, Debug)]
    pub struct DupsEntry {
        pub filename: String,
        pub duplicates: Vec<String>,
    }
pub fn compare_hashes(afile: String, img_hash_list: Vec<ImgHashStruct> ) -> DupsEntry {
    let info = calc_hash(afile.clone());
    let in_filename = info.img_path.clone();
    let in_hash = info.hash.clone();
    let mut duplicates = Vec::new();
    for bfile in img_hash_list.clone() {
        let out_filename = bfile.img_path.clone();
        let out_hash = bfile.hash.clone();
        if in_filename != out_filename {
            let hammer = in_hash.dist(&out_hash);
            if hammer < 8 {
                duplicates.push(out_filename.clone());
            }
        };
    }





        let dups_entry = DupsEntry {
            filename: in_filename.clone(),
            duplicates: duplicates.clone(),
        };
        println!("dups_entry: {:#?}", dups_entry);

        dups_entry
}
