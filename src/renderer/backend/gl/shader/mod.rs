use crate::renderer::*;  // XXX bug?

// Not auto-deriving debug fixes the problem.
#[derive(Debug)]
pub struct ShaderModule {
    pub stage: ShaderStageFlags,
}

