//! `VK_NV_cooperative_vector`, bound by hand.
//!
//! ash 0.38 has no bindings for this extension: no `StructureType` variant, no `ExtendsDeviceCreateInfo`
//! impl, and no command wrappers. So the one structure a device needs to *enable* it is declared here, its
//! values are taken from the registry rather than guessed, and its `p_next` link is built by hand at the call
//! site - because `push_next` is exactly the ash machinery that does not exist for it.
//!
//! From `vulkan_core.h`, `VK_NV_cooperative_vector` spec version 4:
//!
//! ```text
//! VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV = 1000491000
//! ```
//!
//! The matching `..._PROPERTIES_NV` (1000491001) carries `maxCooperativeVectorComponents` and the supported
//! stages. Reading it needs `vkGetPhysicalDeviceProperties2` with our own struct, which is the next step
//! rather than this one: enabling the extension is what a renderer needs before it can compile a
//! cooperative-vector shader at all, and the width it may use is a question asked after that.

use core::ffi::c_void;

/// `VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME`.
pub const NV_COOPERATIVE_VECTOR_NAME: &core::ffi::CStr = c"VK_NV_cooperative_vector";

/// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV`.
pub const S_TYPE_FEATURES_NV: i32 = 1_000_491_000;

/// `VkPhysicalDeviceCooperativeVectorFeaturesNV`, mirrored field for field.
#[repr(C)]
#[derive(Debug)]
pub struct CooperativeVectorFeaturesNV {
    pub s_type: i32,
    pub p_next: *const c_void,
    /// What a shader needs to run the NTC inference: the sampling side.
    pub cooperative_vector: u32,
    /// Training from a shader, which nothing here does. Part of the struct, so it is named.
    pub cooperative_vector_training: u32,
}

// The pointer is `p_next`, which is a link in a chain that lives for one `vkCreateDevice` call. ash marks its
// own structures the same way, for the same reason: the pointer is not shared state, it is a link.
unsafe impl Send for CooperativeVectorFeaturesNV {}
unsafe impl Sync for CooperativeVectorFeaturesNV {}

impl CooperativeVectorFeaturesNV {
    pub fn new(cooperative_vector: bool) -> Self {
        Self {
            s_type: S_TYPE_FEATURES_NV,
            p_next: core::ptr::null(),
            cooperative_vector: cooperative_vector as u32,
            cooperative_vector_training: 0,
        }
    }
}
