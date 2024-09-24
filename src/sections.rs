pub mod about_me;
pub mod hello_world;

#[derive(Clone, Copy, Debug)]
pub enum ActiveSection {
    AboutMe,
    HelloWorld,
}
