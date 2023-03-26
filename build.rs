use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder};

fn main() {
    let result = SpirvBuilder::new("rust_gpu_shaders", "spirv-unknown-spv1.4")
        .print_metadata(MetadataPrintout::DependencyOnly)
        .multimodule(true)
        .capability(Capability::MeshShadingEXT)
        .extension("SPV_EXT_mesh_shader")
        .build()
        .unwrap();

    panic!("{:?}", result.module.unwrap_multi());
}
