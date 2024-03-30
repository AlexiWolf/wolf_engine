pub async fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {}
}

pub struct GraphicsContextBuilder {}

impl GraphicsContextBuilder {
    pub fn build(self) -> GraphicsContext {
        GraphicsContext {}
    }
}

pub struct GraphicsContext {}

#[cfg(test)]
mod graphics_init_tests {
    #[pollster::test]
    async fn should_use_builder_pattern() {
        let graphics_context = crate::init().build().await.unwrap();
    }
}
