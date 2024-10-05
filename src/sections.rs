pub mod about_me;
pub mod hello_world;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveSection {
    AboutMe,
    HelloWorld,
}

impl ActiveSection {
    pub fn all_routes() -> Vec<Vec<String>> {
        vec![
            ActiveSection::AboutMe.into(),
            ActiveSection::HelloWorld.into(),
        ]
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
            ActiveSection::HelloWorld => vec!["hello_world.html".to_string()],
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
            ["about_me.html"] => Ok(Self::AboutMe),
            ["hello_world.html"] => Ok(Self::HelloWorld),

            _ => Err(format!("Couldn't navigate to {:?}", values)),
        }
    }
}
