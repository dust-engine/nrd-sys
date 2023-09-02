use std::{
    ffi::{c_char, c_void},
    fmt::Debug,
};

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
    pub(crate) allocate:
        extern "C" fn(user_arg: *const c_void, size: usize, alignment: usize) -> *mut c_void,
    pub(crate) reallocate: extern "C" fn(
        user_arg: *const c_void,
        memory: *mut c_void,
        old_size: usize,
        old_alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut c_void,
    pub(crate) free:
        extern "C" fn(user_arg: *const c_void, memory: *mut c_void, size: usize, alignment: usize),
    pub(crate) user_arg: *const c_void,
}

#[repr(C)]
pub(crate) struct InstanceCreationDesc {
    pub memory_allocator_interface: MemoryAllocatorInterface,
    pub denoisers: *const DenoiserDesc,
    pub denoisers_num: u32,
}

#[repr(u32)]
#[derive(Debug)]
pub enum Sampler {
    NearestClamp,
    NearestMirroredRepeat,
    LinearClamp,
    LinearMirroredRepeat,

    MaxNum,
}

#[repr(C)]
#[derive(Debug)]
pub struct ComputeShaderDesc {
    bytecode: *const c_void,
    size: u64,
}

#[repr(u32)]
pub enum DescriptorType {
    // read-only, SRV
    Texture,

    // read-write, UAV
    StorageTexture,
}

#[repr(C)]
struct ResourceRangeDesc {
    descriptor_type: DescriptorType,
    base_register_index: u32,
    descriptors_num: u32,
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Format {
    R8_UNORM,
    R8_SNORM,
    R8_UINT,
    R8_SINT,

    RG8_UNORM,
    RG8_SNORM,
    RG8_UINT,
    RG8_SINT,

    RGBA8_UNORM,
    RGBA8_SNORM,
    RGBA8_UINT,
    RGBA8_SINT,
    RGBA8_SRGB,

    R16_UNORM,
    R16_SNORM,
    R16_UINT,
    R16_SINT,
    R16_SFLOAT,

    RG16_UNORM,
    RG16_SNORM,
    RG16_UINT,
    RG16_SINT,
    RG16_SFLOAT,

    RGBA16_UNORM,
    RGBA16_SNORM,
    RGBA16_UINT,
    RGBA16_SINT,
    RGBA16_SFLOAT,

    R32_UINT,
    R32_SINT,
    R32_SFLOAT,

    RG32_UINT,
    RG32_SINT,
    RG32_SFLOAT,

    RGB32_UINT,
    RGB32_SINT,
    RGB32_SFLOAT,

    RGBA32_UINT,
    RGBA32_SINT,
    RGBA32_SFLOAT,

    R10_G10_B10_A2_UNORM,
    R10_G10_B10_A2_UINT,
    R11_G11_B10_UFLOAT,
    R9_G9_B9_E5_UFLOAT,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineDesc {
    pub compute_shader_dxbc: ComputeShaderDesc,
    pub compute_shader_dxil: ComputeShaderDesc,
    pub compute_shader_spirv: ComputeShaderDesc,
    shader_file_name: *const c_char,
    shader_entry_point_name: *const c_char,
    resource_ranges: *const ResourceRangeDesc,
    resource_ranges_num: u32,
    pub has_constant_data: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct TextureDesc {
    pub format: Format,
    pub width: u16,
    pub height: u16,
    pub mip_num: u16,
}

#[repr(C)]
#[derive(Debug)]
struct DescriptorPoolDesc {
    sets_max_num: u32,
    constant_buffers_max_num: u32,
    samplers_max_num: u32,
    textures_max_num: u32,
    storage_textures_max_num: u32,
}

#[repr(C)]
pub struct InstanceDesc {
    // Constant buffer (shared)
    constant_buffer_max_data_size: u32,
    constant_buffer_space_index: u32,
    constant_buffer_register_index: u32,

    // Samplers (shared)
    samplers: *const Sampler,
    samplers_num: u32,
    samplers_space_index: u32,
    samplers_base_register_index: u32,

    // Pipelines
    // - if "PipelineDesc::hasConstantData = true" a pipeline has a constant buffer with the shared description
    // - if "samplers" are used as static/immutable samplers, "DescriptorPoolDesc::samplerMaxNum" is not needed (it counts samplers across all dispatches)
    pipelines: *const PipelineDesc,
    pipelines_num: u32,
    resources_space_index: u32,

    // Textures
    permanent_pool: *const TextureDesc,
    permanent_pool_size: u32,
    transient_pool: *const TextureDesc,
    transient_pool_size: u32,

    // Limits
    descriptor_pool_desc: DescriptorPoolDesc,
}
impl InstanceDesc {
    pub fn samplers(&self) -> &[Sampler] {
        unsafe { std::slice::from_raw_parts(self.samplers, self.samplers_num as usize) }
    }
    pub fn pipelines(&self) -> &[PipelineDesc] {
        unsafe { std::slice::from_raw_parts(self.pipelines, self.pipelines_num as usize) }
    }
    pub fn permanent_pool(&self) -> &[TextureDesc] {
        unsafe {
            std::slice::from_raw_parts(self.permanent_pool, self.permanent_pool_size as usize)
        }
    }
    pub fn transient_pool(&self) -> &[TextureDesc] {
        unsafe {
            std::slice::from_raw_parts(self.transient_pool, self.transient_pool_size as usize)
        }
    }
}

impl Debug for InstanceDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstanceDesc")
            .field(
                "constant_buffer_max_data_size",
                &self.constant_buffer_max_data_size,
            )
            .field(
                "constant_buffer_space_index",
                &self.constant_buffer_space_index,
            )
            .field(
                "constant_buffer_register_index",
                &self.constant_buffer_register_index,
            )
            .field("samplers", &self.samplers())
            .field("samplers_space_index", &self.samplers_space_index)
            .field(
                "samplers_base_register_index",
                &self.samplers_base_register_index,
            )
            .field("pipelines", &self.pipelines())
            .field("resources_space_index", &self.resources_space_index)
            .field("permanent_pool", &self.permanent_pool())
            .field("transient_pool", &self.transient_pool())
            .field("descriptor_pool_desc", &self.descriptor_pool_desc)
            .finish()
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum AccumulationMode {
    // Common mode (accumulation continues normally)
    Continue,

    // Discards history and resets accumulation
    Restart,

    // Like RESTART, but additionally clears resources from potential garbage
    ClearAndRestart,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommonSettings {
    // Matrix requirements:
    //     - usage - vector is a column
    //     - layout - column-major
    //     - non jittered!
    // LH / RH projection matrix (INF far plane is supported) with non-swizzled rows, i.e. clip-space depth = z / w
    view_to_clip_matrix: [f32; 16],

    // Previous projection matrix
    view_to_clip_matrix_prev: [f32; 16],

    // World-space to camera-space matrix
    world_to_view_matrix: [f32; 16],

    // If coordinate system moves with the camera, camera delta must be included to reflect camera motion
    world_to_view_matrix_prev: [f32; 16],

    // (Optional) Previous world-space to current world-space matrix. It is for virtual normals, where a coordinate
    // system of the virtual space changes frame to frame, such as in a case of animated intermediary reflecting
    // surfaces when primary surface replacement is used for them.
    world_prev_to_world_matrix: [f32; 16],

    // used as "IN_MV * motionVectorScale" (use .z = 0 for 2D screen-space motion)
    motion_vector_scale: [f32; 3],

    // [-0.5; 0.5] - sampleUv = pixelUv + cameraJitter
    camera_jitter: [f32; 2],
    camera_jitter_prev: [f32; 2],

    // (0; 1] - dynamic resolution scaling
    resolution_scale: [f32; 2],
    resolution_scale_prev: [f32; 2],

    // (ms) - user provided if > 0, otherwise - tracked internally
    time_delta_between_frames: f32,

    // (units) > 0 - use TLAS or tracing range (max value = NRD_FP16_MAX / NRD_FP16_VIEWZ_SCALE - 1 = 524031)
    denoising_range: f32,

    // (normalized %) - if relative distance difference is greater than threshold, history gets reset (0.5-2.5% works well)
    disocclusion_threshold: f32,

    // (normalized %) - alternative disocclusion threshold, which is mixed to based on IN_DISOCCLUSION_THRESHOLD_MIX
    disocclusion_threshold_alternate: f32,

    // [0; 1] - enables "noisy input / denoised output" comparison
    split_screen: f32,

    // For internal needs
    debug: f32,

    // (pixels) - data rectangle origin in ALL input textures
    input_subrect_origin: [u32; 2],

    // A consecutive number
    frame_index: u32,

    // To reset history set to RESTART / CLEAR_AND_RESTART for one frame
    accumulation_mode: AccumulationMode,

    // If "true" IN_MV is 3D motion in world-space (0 should be everywhere if the scene is static),
    // otherwise it's 2D (+ optional Z delta) screen-space motion (0 should be everywhere if the camera doesn't move) (recommended value = true)
    is_motion_vector_in_world_space: bool,

    // If "true" IN_DIFF_CONFIDENCE and IN_SPEC_CONFIDENCE are available
    is_history_confidence_available: bool,

    // If "true" IN_DISOCCLUSION_THRESHOLD_MIX is available
    is_disocclusion_threshold_mix_available: bool,

    // If "true" IN_BASECOLOR_METALNESS is available
    is_base_color_metalness_available: bool,

    // Enables debug overlay in OUT_VALIDATION, requires "InstanceCreationDesc::allowValidation = true"
    enable_validation: bool,
}

#[allow(non_snake_case)]
extern "fastcall" {
    pub(crate) fn GetLibraryDesc() -> &'static LibraryDesc;
    pub(crate) fn CreateInstance(desc: &InstanceCreationDesc, instance: &mut *mut c_void)
        -> Result;
    pub(crate) fn DestroyInstance(instance: *mut c_void);
    pub(crate) fn GetInstanceDesc(instance: *mut c_void) -> *const InstanceDesc;
    pub(crate) fn SetCommonSettings(instance: *mut c_void, settings: &CommonSettings) -> Result;
}
