extern crate img_hash;
use crate::factory;
use img_hash::{HasherConfig, ImageHash};
use serde::Serialize;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct ImgHashStruct {
    pub img_path: String,
    pub hash: ImageHash,
}
pub fn calc_hash(apath: String) -> ImgHashStruct {
    let hasher_config = HasherConfig::new().to_hasher();
    let image = image::open(apath.clone()).expect(apath.clone().as_str());


    let hashed = hasher_config.hash_image(&image);
    let imghash = ImgHashStruct {
        img_path: apath.clone(),
        hash: hashed,
    };

    imghash
}

pub fn calc_hash_test(apath: String) -> bool {
    // let hasher_config = HasherConfig::new().to_hasher();
    let image_results = image::open(apath.clone());

    let _image = match image_results {
        Ok(_image) => return true,
        Err(e) => {
            println!("Error: {}", e);
            return false;
        }
    };
}

#[derive(Serialize, Clone, Debug)]
pub struct DupsEntry {
    pub filename: String,
    pub duplicates: Vec<String>,
}
pub fn compare_hashes(afile: String, img_hash_list: Vec<ImgHashStruct>) -> DupsEntry {
    let calc_test = calc_hash_test(afile.clone());
    println!("calc_test: \n\t{}", calc_test);
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
    pub duplicates: Vec<DupStruct>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DupStruct {
    pub strdups: String,
    pub httpdups: String,
}

fn transform_dup_entry_struct(dups_entry: DupsEntry) -> TransDupsEntry {
    let filename = dups_entry.filename.clone();
    let filename_parts = filename.split("/").collect::<Vec<&str>>();
    let fname = filename_parts.len() - 1;
    let http_filename = "http://192.168.0.91:8181/image/".to_string() + filename_parts[fname];
    let mut comp_duplicates = Vec::new();
    for dup in dups_entry.duplicates.clone() {
        let dup_parts = dup.split("/").collect::<Vec<&str>>();
        let dp = dup_parts.len() - 1;
        let http_dup = "http://192.168.0.91:8181/image/".to_string() + dup_parts[dp];
        let dupsstruct = DupStruct {
            strdups: dup.clone(),
            httpdups: http_dup.clone(),
        };
        comp_duplicates.push(dupsstruct);
    }

    let trans_dup_entry = TransDupsEntry {
        filename: filename.clone(),
        httpfilename: http_filename.clone(),
        duplicates: comp_duplicates.clone(),
    };

    println!("dups_entry: {:#?}", trans_dup_entry);

    trans_dup_entry
}
