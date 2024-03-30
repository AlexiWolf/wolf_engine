pub fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {}
}

pub struct GraphicsContextBuilder {}

impl GraphicsContextBuilder {
    pub async fn build(self) -> Result<GraphicsContext, &'static str> {
        Ok(GraphicsContext {})
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
