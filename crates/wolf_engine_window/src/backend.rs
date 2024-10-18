use crate::WindowContext;

pub trait WindowSystem {
    fn context(&self) -> WindowContext;
}
