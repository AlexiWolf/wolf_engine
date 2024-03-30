pub async fn init() -> Result<GraphicsContext, &'static str> {
    Ok(GraphicsContext {})
}

pub struct GraphicsContext {}

#[cfg(test)]
mod graphics_init_tests {
    #[test]
    fn should_use_builder_pattern() {
        let graphics_context = crate::init().build().unwrap();
    }
}
