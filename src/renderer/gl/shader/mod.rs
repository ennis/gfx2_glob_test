// Explicitly importing ShaderStageFlags fixes the issue
use crate::renderer::*; // use crate::renderer::ShaderStageFlags;

// Not auto-deriving debug fixes the problem.
#[derive(Debug)]
pub struct ShaderModule {
    pub stage: ShaderStageFlags,
}

