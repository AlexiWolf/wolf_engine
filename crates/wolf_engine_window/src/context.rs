use std::sync::Arc;

use crate::{backend::WindowBackendAdapter, settings::WindowSettings, Window};

pub struct Context {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
}

impl Context {
    pub(crate) fn new(backend_adapter: Box<dyn WindowBackendAdapter>) -> Self {
        Self {
            backend_adapter: Arc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    pub(crate) fn backend_adapter(&self) -> Arc<Box<dyn WindowBackendAdapter>> {
        self.backend_adapter.clone()
    }
}
