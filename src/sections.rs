pub mod about_me;
pub mod hello_world;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveSection {
    AboutMe,
    HelloWorld,
}

impl From<ActiveSection> for String {
    fn from(active_section: ActiveSection) -> String {
        let active_section: Vec<String> = active_section.into();
        active_section.join("/");
        return "/".to_string() + active_section.join("/").as_str();
    }
}

impl From<ActiveSection> for Vec<String> {
    fn from(active_section: ActiveSection) -> Vec<String> {
        match active_section {
            ActiveSection::AboutMe => vec!["about_me".to_string()],
            ActiveSection::HelloWorld => vec!["hello_world".to_string()],
        }
    }
}

impl TryFrom<&[String]> for ActiveSection {
    type Error = String;
    fn try_from(values: &[String]) -> Result<Self, Self::Error> {
        let mut str_vec = vec![];
        for value in values {
            str_vec.push(value.as_str());
        }

        match str_vec.as_slice() {
            ["about_me"] => Ok(Self::AboutMe),
            ["hello_world"] => Ok(Self::HelloWorld),

            _ => Err(format!("Couldn't navigate to {:?}", values)),
        }
    }
}
