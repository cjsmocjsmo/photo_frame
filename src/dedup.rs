extern crate img_hash;
use crate::factory;
use img_hash::HasherConfig;
use img_hash::ImageHash;
use serde::Serialize;
use std::io::Write;

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

#[derive(Serialize, Clone, Debug)]
pub struct DupsEntry {
    pub filename: String,
    pub duplicates: Vec<String>,
}
pub fn compare_hashes(afile: String, img_hash_list: Vec<ImgHashStruct>) -> DupsEntry {
    let info = calc_hash(afile.clone());
    let in_filename = info.img_path.clone();
    let in_hash = info.hash.clone();
    let mut duplicates = Vec::new();
    for bfile in img_hash_list.clone() {
        let out_filename = bfile.img_path.clone();
        let out_hash = bfile.hash.clone();
        if in_filename != out_filename {
            let hammer = in_hash.dist(&out_hash);
            if hammer < 5 {
                println!("hammer: {}", hammer);
                duplicates.push(out_filename.clone());
            }
        };
    }

    let dups_entry = DupsEntry {
        filename: in_filename.clone(),
        duplicates: duplicates.clone(),
    };

    if duplicates.len() > 0 {

        let transform = transform_dup_entry_struct(dups_entry.clone());

        let json = serde_json::to_string(&transform).unwrap();
        let f = factory::Factory {
            path: afile.clone(),
        };
        let ddoutfile = f.create_dedup_output_file();

        let mut output_file_results = std::fs::File::create(ddoutfile).unwrap();
        output_file_results.write_all(json.as_bytes()).unwrap();
    }


    dups_entry
}

#[derive(Serialize, Clone, Debug)]
pub struct TransDupsEntry {
    pub filename: String,
    pub httpfilename: String,
    pub duplicates: Vec<String>,
    pub httpduplicates: Vec<String>,
}
fn transform_dup_entry_struct(dups_entry: DupsEntry) -> TransDupsEntry {
    let filename = dups_entry.filename.clone();
    let filename_parts = filename.split("/").collect::<Vec<&str>>();
    println!("filename_parts: {:#?}", filename_parts);
    let foo = println!("http://192.168.0.91:8181/image/{}", filename_parts[1]);
    println!("foo: {:#?}", foo);
    let http_filename = format!("http://192.168.0.91:8181/image/{}", filename_parts[1]);
    println!("http_filename: {}", http_filename);
    let duplicates = dups_entry.duplicates.clone();

    let mut http_duplicates = Vec::new();
    for dup in dups_entry.duplicates.clone() {
        let dup_parts = dup.split("/").collect::<Vec<&str>>();
        let http_dup = format!("http://192.168.0.91:8181/image/{}", dup_parts[1]);
        println!("http_dup: {}", http_dup);
        http_duplicates.push(http_dup);
    }

    let trans_dup_entry = TransDupsEntry {
        filename: filename.clone(),
        httpfilename: http_filename.clone(),
        duplicates: duplicates.clone(),
        httpduplicates: http_duplicates.clone(),
    };

    // println!("dups_entry: {:#?}", trans_dup_entry);

    trans_dup_entry
}