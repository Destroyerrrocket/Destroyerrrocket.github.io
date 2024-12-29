use std::io::Write;
use std::{env, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BlogEntry {
    blog_date: String, // YYYY-MM-DD
    title: String,
    description: String,
    image_file: String,
}

fn main() {
    println!("cargo::rerun-if-changed=blogs/info.json");
    println!("cargo::rerun-if-changed=build.rs");

    let entries = fs::read_to_string("blogs/info.json").unwrap();

    let deserialized: Vec<BlogEntry> = serde_json::from_str(&entries).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("current_blogs.rs");

    let mut file = std::io::BufWriter::new(fs::File::create(&dest_path).unwrap());

    writeln!(file, "const BLOGS: [BlogEntry; {}] = [", deserialized.len()).unwrap();
    for entry in deserialized {
        let date_fields = entry
            .blog_date
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if date_fields.len() != 3 {
            panic!("Invalid date format");
        }
        writeln!(file, "BlogEntry {{").unwrap();
        writeln!(file, "blog_date: crate::sections::BlogDate {{").unwrap();
        writeln!(file, "year: {},", date_fields[0]).unwrap();
        writeln!(file, "month: {},", date_fields[1]).unwrap();
        writeln!(file, "day: {},", date_fields[2]).unwrap();
        writeln!(file, "}},").unwrap();
        writeln!(file, "title: \"{}\",", entry.title).unwrap();
        writeln!(file, "description: \"{}\",", entry.description).unwrap();
        writeln!(
            file,
            "image_file_thumbnail: asset!(\"{}\",",
            entry.image_file
        )
        .unwrap();
        writeln!(
            file,
            "ImageAssetOptions::new().with_size(ImageSize::Manual {{width: 384, height: 384}}).with_avif()),",
        )
        .unwrap();
        writeln!(file, "image_file_blog: asset!(\"{}\",", entry.image_file).unwrap();
        writeln!(file, "ImageAssetOptions::new().with_avif()),",).unwrap();
        writeln!(file, "}},").unwrap();
    }
    writeln!(file, "];").unwrap();
}
