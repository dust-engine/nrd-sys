use std::{ffi::c_void, fmt::Debug};

#[repr(C)]
#[derive(Debug)]
pub struct SPIRVBindingOffsets {
    sampler_offset: u32,
    texture_offset: u32,
    constant_buffer_offset: u32,
    storage_texture_and_buffer_offset: u32,
}

#[repr(u32)]
#[derive(Debug)]
pub enum Denoiser {
    // =============================================================================================================================
    // REBLUR
    // =============================================================================================================================

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_RADIANCE_HITDIST,
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE
    // OUTPUTS - OUT_DIFF_RADIANCE_HITDIST
    ReblurDiffuse,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_HITDIST,
    // OUTPUTS - OUT_DIFF_HITDIST
    ReblurDiffuseOcclusion,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_SH0, IN_DIFF_SH1
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE
    // OUTPUTS - OUT_DIFF_SH0, OUT_DIFF_SH1
    ReblurDiffuseSh,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_RADIANCE_HITDIST,
    // OPTIONAL INPUTS - IN_SPEC_DIRECTION_PDF, IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_SPEC_RADIANCE_HITDIST
    ReblurSpecular,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_HITDIST,
    // OUTPUTS - OUT_SPEC_HITDIST
    ReblurSpecularOcclusion,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_SH0, IN_SPEC_SH1
    // OPTIONAL INPUTS - IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_SPEC_SH0, OUT_SPEC_SH1
    ReblurSpecularSh,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_RADIANCE_HITDIST, IN_SPEC_RADIANCE_HITDIST,
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE,  IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_DIFF_RADIANCE_HITDIST, OUT_SPEC_RADIANCE_HITDIST
    ReblurDiffuseSpecular,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_HITDIST, IN_SPEC_HITDIST,
    // OUTPUTS - OUT_DIFF_HITDIST, OUT_SPEC_HITDIST
    ReblurDiffuseSpecularOcclusion,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_SH0, IN_DIFF_SH1, IN_SPEC_SH0, IN_SPEC_SH1
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE,  IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_DIFF_SH0, OUT_DIFF_SH1, OUT_SPEC_SH0, OUT_SPEC_SH1
    ReblurDiffuseSpecularSh,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_DIRECTION_HITDIST,
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE
    // OUTPUTS - OUT_DIFF_DIRECTION_HITDIST
    ReblurDiffuseDirectionalOcclusion,

    // =============================================================================================================================
    // SIGMA
    // =============================================================================================================================

    // INPUTS - IN_NORMAL_ROUGHNESS, IN_SHADOWDATA, OUT_SHADOW_TRANSLUCENCY (used as history)
    // OUTPUTS - OUT_SHADOW_TRANSLUCENCY
    SigmaShadow,

    // INPUTS - IN_NORMAL_ROUGHNESS, IN_SHADOWDATA, IN_SHADOW_TRANSLUCENCY, OUT_SHADOW_TRANSLUCENCY (used as history)
    // OUTPUTS - OUT_SHADOW_TRANSLUCENCY
    SigmaShadowTranslucency,

    // =============================================================================================================================
    // RELAX
    // =============================================================================================================================

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_RADIANCE_HITDIST
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE
    // OUTPUTS - OUT_DIFF_RADIANCE_HITDIST
    RelaxDiffuse,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_SH0, IN_DIFF_SH1
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE
    // OUTPUTS - OUT_DIFF_SH0, OUT_DIFF_SH1
    RelaxDiffuseSh,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_RADIANCE_HITDIST
    // OPTIONAL INPUTS - IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_SPEC_RADIANCE_HITDIST
    RelaxSpecular,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_SH0, IN_SPEC_SH1
    // OPTIONAL INPUTS - IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_SPEC_SH0, OUT_SPEC_SH1
    RelaxSpecularSh,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_RADIANCE_HITDIST, IN_SPEC_RADIANCE_HITDIST
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE,  IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_DIFF_RADIANCE_HITDIST, OUT_SPEC_RADIANCE_HITDIST
    RelaxDiffuseSpecular,

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_DIFF_SH0, IN_DIFF_SH1, IN_SPEC_SH0, IN_SPEC_SH1
    // OPTIONAL INPUTS - IN_DIFF_CONFIDENCE,  IN_SPEC_CONFIDENCE
    // OUTPUTS - OUT_DIFF_SH0, OUT_DIFF_SH1, OUT_SPEC_SH0, OUT_SPEC_SH1
    RelaxDiffuseSpecularSh,

    // =============================================================================================================================
    // REFERENCE
    // =============================================================================================================================

    // INPUTS - IN_RADIANCE
    // OUTPUTS - OUT_RADIANCE
    REFERENCE,

    // =============================================================================================================================
    // MOTION VECTORS
    // =============================================================================================================================

    // INPUTS - IN_MV, IN_NORMAL_ROUGHNESS, IN_VIEWZ, IN_SPEC_HITDIST
    // OUTPUTS - OUT_REFLECTION_MV
    SpecularReflectionMv,

    // INPUTS - IN_MV, IN_DELTA_PRIMARY_POS, IN_DELTA_SECONDARY_POS
    // OUTPUT - OUT_DELTA_MV
    SpecularDeltaMv,
}

#[repr(u8)]
#[derive(Debug)]
pub enum NormalEncoding {
    // Worst IQ on curved (not bumpy) surfaces
    Rgba8Unorm,
    Rgba8Snorm,

    // Moderate IQ on curved (not bumpy) surfaces, but offers optional materialID support (normals are oct-packed)
    R10G10B10A2Unorm,

    // Best IQ on curved (not bumpy) surfaces
    Rgba16Unorm,
    Rgba16Snorm, // can be used with FP formats
}

/// NRD_ROUGHNESS_ENCODING variants
#[repr(u8)]
#[derive(Debug)]
pub enum RoughnessEncoding {
    // Alpha (m)
    SqLinear,

    // Linear roughness (best choice)
    LINEAR,

    // Sqrt(linear roughness)
    SqrtLinear,
}

#[repr(C)]
pub struct LibraryDesc {
    pub spirv_binding_offsets: SPIRVBindingOffsets,
    supported_denoisers: *const Denoiser,
    supported_denoisers_num: u32,
    pub version_major: u8,
    pub version_minor: u8,
    pub version_build: u8,
    pub normal_encoding: NormalEncoding,
    pub roughness_encoding: RoughnessEncoding,
}

impl LibraryDesc {
    pub fn supported_denoisers(&self) -> &[Denoiser] {
        unsafe {
            std::slice::from_raw_parts(
                self.supported_denoisers,
                self.supported_denoisers_num as usize,
            )
        }
    }
}

impl Debug for LibraryDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryDesc")
            .field("spirv_binding_offsets", &self.spirv_binding_offsets)
            .field("supported_denoisers", &self.supported_denoisers())
            .field("version_major", &self.version_major)
            .field("version_minor", &self.version_minor)
            .field("version_build", &self.version_build)
            .field("normal_encoding", &self.normal_encoding)
            .field("roughness_encoding", &self.roughness_encoding)
            .finish()
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum Result {
    Success,
    Failure,
    InvalidArgument,
    Unsupported,
    NonUniqueIdentifier,
}

#[repr(transparent)]
pub struct Identifier(u32);

#[repr(C)]
pub struct DenoiserDesc {
    identifier: u32,
    denoiser: Denoiser,
    render_width: u16,
    render_height: u16,
}

#[repr(C)]
pub(crate) struct MemoryAllocatorInterface {
    pub(crate) allocate: extern "C" fn(user_arg: *const c_void, size: usize, alignment: usize) -> *mut c_void,
    pub(crate) reallocate: extern "C" fn(
        user_arg: *const c_void,
        memory: *mut c_void,
        old_size: usize,
        old_alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut c_void,
    pub(crate) free: extern "C" fn(user_arg: *const c_void, memory: *mut c_void, size: usize, alignment: usize),
    pub(crate) user_arg: *const c_void,
}

#[repr(C)]
pub(crate) struct InstanceCreationDesc {
    pub memory_allocator_interface: MemoryAllocatorInterface,
    pub denoisers: *const DenoiserDesc,
    pub denoisers_num: u32,
}

#[allow(non_snake_case)]
extern "fastcall" {
    pub(crate) fn GetLibraryDesc() -> &'static LibraryDesc;
    pub(crate) fn CreateInstance(desc: &InstanceCreationDesc, instance: &mut *mut c_void) -> Result;
    pub(crate) fn DestroyInstance(instance: *mut c_void);
}
