pub mod about_me;
pub mod blog;
pub mod password_generator;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlogDate {
    year: u64,
    month: u8,
    day: u8,
}

impl std::fmt::Display for BlogDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.year, self.month, self.day)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveSection {
    AboutMe,
    PasswordGenerator,
    Blog(Option<BlogDate>),
}

impl ActiveSection {
    pub fn all_routes() -> Vec<Vec<String>> {
        [
            ActiveSection::AboutMe,
            ActiveSection::PasswordGenerator,
            ActiveSection::Blog(None),
        ]
        .into_iter()
        .map(|section| section.into())
        .collect()
    }

    pub fn all_static_routes() -> Vec<Vec<String>> {
        [ActiveSection::AboutMe, ActiveSection::Blog(None)]
            .into_iter()
            .map(|section| section.into())
            .collect()
    }
}

impl From<ActiveSection> for String {
    fn from(active_section: ActiveSection) -> String {
        let active_section: Vec<String> = active_section.into();
        active_section.join("/");
        "/".to_string() + active_section.join("/").as_str()
    }
}

impl From<ActiveSection> for Vec<String> {
    fn from(active_section: ActiveSection) -> Vec<String> {
        match active_section {
            ActiveSection::AboutMe => vec!["about_me.html".to_string()],
            ActiveSection::PasswordGenerator => vec!["password_generator.html".to_string()],
            ActiveSection::Blog(None) => vec!["blog.html".to_string()],
            ActiveSection::Blog(Some(blog_date)) => vec![
                "blog".to_string(),
                blog_date.year.to_string(),
                blog_date.month.to_string(),
                blog_date.day.to_string(),
                "index.html".to_string(),
            ],
        }
    }
}

fn parse_blog_date(year: &str, month: &str, day: &str) -> Option<BlogDate> {
    let year: u64 = year.parse().ok()?;
    let month: u8 = month.parse().ok()?;
    let day: u8 = day.parse().ok()?;
    Some(BlogDate { year, month, day })
}

impl TryFrom<&[String]> for ActiveSection {
    type Error = String;
    fn try_from(values: &[String]) -> Result<Self, Self::Error> {
        let mut str_vec = vec![];
        for value in values {
            str_vec.push(value.as_str());
        }

        match str_vec.as_slice() {
            ["about_me.html"] => Ok(Self::AboutMe),
            ["password_generator.html"] => Ok(Self::PasswordGenerator),
            ["blog.html"] => Ok(Self::Blog(None)),
            ["blog", year, month, day, "index.html"] => {
                Ok(Self::Blog(parse_blog_date(year, month, day)))
            }
            _ => Err(format!("Couldn't navigate to {:?}", values)),
        }
    }
}
