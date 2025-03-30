use std::{collections::HashMap, sync::LazyLock};

use manganis::*;

#[derive(Clone, PartialEq)]
pub struct BlogEntry {
    pub blog_date: crate::sections::BlogDate,
    pub title: &'static str,
    pub description: &'static str,
    pub image_file_thumbnail: Asset,
    pub image_file_blog: Asset,
    pub html: Asset,
}

include!(concat!(env!("OUT_DIR"), "/current_blogs.rs"));

pub fn get_blogs() -> &'static HashMap<crate::sections::BlogDate, BlogEntry> {
    static MAP_BLOGS: LazyLock<HashMap<crate::sections::BlogDate, BlogEntry>> =
        LazyLock::new(|| {
            BLOGS
                .into_iter()
                .map(|blog| (blog.blog_date, blog))
                .collect()
        });
    &MAP_BLOGS
}

pub fn get_blog(date: crate::sections::BlogDate) -> &'static BlogEntry {
    get_blogs().get(&date).unwrap()
}
