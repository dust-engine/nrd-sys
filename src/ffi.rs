use std::{
    ffi::{c_char, c_void, CStr},
    fmt::Debug,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SPIRVBindingOffsets {
    pub sampler_offset: u32,
    pub texture_offset: u32,
    pub constant_buffer_offset: u32,
    pub storage_texture_and_buffer_offset: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
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
    Reference,

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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum Result {
    Success,
    Failure,
    InvalidArgument,
    Unsupported,
    NonUniqueIdentifier,
}

impl Result {
    pub fn ok<T>(self, value: T) -> std::result::Result<T, Result> {
        match self {
            Result::Success => Ok(value),
            _ => Err(self),
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Identifier(pub u32);

#[repr(C)]
pub struct DenoiserDesc {
    pub identifier: Identifier,
    pub denoiser: Denoiser,
    pub render_width: u16,
    pub render_height: u16,
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
#[derive(Debug, Clone, Copy)]
pub enum Sampler {
    NearestClamp,
    NearestMirroredRepeat,
    LinearClamp,
    LinearMirroredRepeat,
}

#[repr(C)]
pub struct ComputeShaderDesc {
    bytecode: *const c_void,
    size: u64,
}
impl Debug for ComputeShaderDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ComputeShaderDesc({} bytes)", self.size))
    }
}
impl std::ops::Deref for ComputeShaderDesc {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.bytecode as *const u8, self.size as usize) }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    // read-only, SRV
    Texture,

    // read-write, UAV
    StorageTexture,
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceRangeDesc {
    pub descriptor_type: DescriptorType,
    pub base_register_index: u32,
    pub descriptors_num: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
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
pub struct PipelineDesc {
    pub compute_shader_dxbc: ComputeShaderDesc,
    pub compute_shader_dxil: ComputeShaderDesc,
    pub compute_shader_spirv: ComputeShaderDesc,
    shader_file_name: *const c_char,
    shader_entry_point_name: *const c_char,
    resource_ranges: *const ResourceRangeDesc,
    resource_ranges_num: u32,
    pub max_repeat_num: u32,
    pub has_constant_data: bool,
}
impl PipelineDesc {
    pub fn shader_file_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.shader_file_name) }
    }
    pub fn shader_entry_point_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.shader_entry_point_name) }
    }
    pub fn resource_ranges(&self) -> &[ResourceRangeDesc] {
        unsafe {
            std::slice::from_raw_parts(self.resource_ranges, self.resource_ranges_num as usize)
        }
    }
}
impl Debug for PipelineDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineDesc")
            .field("compute_shader_dxbc", &self.compute_shader_dxbc)
            .field("compute_shader_dxil", &self.compute_shader_dxil)
            .field("compute_shader_spirv", &self.compute_shader_spirv)
            .field("shader_file_name", &self.shader_file_name())
            .field("shader_entry_point_name", &self.shader_entry_point_name())
            .field("resource_ranges", &self.resource_ranges())
            .field("max_repeat_num", &self.max_repeat_num)
            .field("has_constant_data", &self.has_constant_data)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureDesc {
    pub format: Format,
    pub width: u16,
    pub height: u16,
    pub mip_num: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorPoolDesc {
    pub sets_max_num: u32,
    pub constant_buffers_max_num: u32,
    pub samplers_max_num: u32,
    pub textures_max_num: u32,
    pub storage_textures_max_num: u32,
}

#[repr(C)]
pub struct InstanceDesc {
    // Constant buffer (shared)
    pub constant_buffer_max_data_size: u32,
    pub constant_buffer_space_index: u32,
    pub constant_buffer_register_index: u32,

    // Samplers (shared)
    samplers: *const Sampler,
    samplers_num: u32,
    pub samplers_space_index: u32,
    pub samplers_base_register_index: u32,

    // Pipelines
    // - if "PipelineDesc::hasConstantData = true" a pipeline has a constant buffer with the shared description
    // - if "samplers" are used as static/immutable samplers, "DescriptorPoolDesc::samplerMaxNum" is not needed (it counts samplers across all dispatches)
    pipelines: *const PipelineDesc,
    pipelines_num: u32,
    pub resources_space_index: u32,

    // Textures
    permanent_pool: *const TextureDesc,
    permanent_pool_size: u32,
    transient_pool: *const TextureDesc,
    transient_pool_size: u32,

    // Limits
    pub descriptor_pool_desc: DescriptorPoolDesc,
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
#[derive(Debug, Clone, Copy)]
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
    pub view_to_clip_matrix: [f32; 16],

    // Previous projection matrix
    pub view_to_clip_matrix_prev: [f32; 16],

    // World-space to camera-space matrix
    pub world_to_view_matrix: [f32; 16],

    // If coordinate system moves with the camera, camera delta must be included to reflect camera motion
    pub world_to_view_matrix_prev: [f32; 16],

    // (Optional) Previous world-space to current world-space matrix. It is for virtual normals, where a coordinate
    // system of the virtual space changes frame to frame, such as in a case of animated intermediary reflecting
    // surfaces when primary surface replacement is used for them.
    pub world_prev_to_world_matrix: [f32; 16],

    // used as "IN_MV * motionVectorScale" (use .z = 0 for 2D screen-space motion)
    pub motion_vector_scale: [f32; 3],

    // [-0.5; 0.5] - sampleUv = pixelUv + cameraJitter
    pub camera_jitter: [f32; 2],
    pub camera_jitter_prev: [f32; 2],

    // (0; 1] - dynamic resolution scaling
    pub resolution_scale: [f32; 2],
    pub resolution_scale_prev: [f32; 2],

    // (ms) - user provided if > 0, otherwise - tracked internally
    pub time_delta_between_frames: f32,

    // (units) > 0 - use TLAS or tracing range (max value = NRD_FP16_MAX / NRD_FP16_VIEWZ_SCALE - 1 = 524031)
    pub denoising_range: f32,

    // (normalized %) - if relative distance difference is greater than threshold, history gets reset (0.5-2.5% works well)
    pub disocclusion_threshold: f32,

    // (normalized %) - alternative disocclusion threshold, which is mixed to based on IN_DISOCCLUSION_THRESHOLD_MIX
    pub disocclusion_threshold_alternate: f32,

    // [0; 1] - enables "noisy input / denoised output" comparison
    pub split_screen: f32,

    // For internal needs
    pub debug: f32,

    // (pixels) - data rectangle origin in ALL input textures
    pub input_subrect_origin: [u32; 2],

    // A consecutive number
    pub frame_index: u32,

    // To reset history set to RESTART / CLEAR_AND_RESTART for one frame
    pub accumulation_mode: AccumulationMode,

    // If "true" IN_MV is 3D motion in world-space (0 should be everywhere if the scene is static),
    // otherwise it's 2D (+ optional Z delta) screen-space motion (0 should be everywhere if the camera doesn't move) (recommended value = true)
    pub is_motion_vector_in_world_space: bool,

    // If "true" IN_DIFF_CONFIDENCE and IN_SPEC_CONFIDENCE are available
    pub is_history_confidence_available: bool,

    // If "true" IN_DISOCCLUSION_THRESHOLD_MIX is available
    pub is_disocclusion_threshold_mix_available: bool,

    // If "true" IN_BASECOLOR_METALNESS is available
    pub is_base_color_metalness_available: bool,

    // Enables debug overlay in OUT_VALIDATION, requires "InstanceCreationDesc::allowValidation = true"
    pub enable_validation: bool,
}
impl Default for CommonSettings {
    fn default() -> Self {
        const IDENTITY: [f32; 16] = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        Self {
            view_to_clip_matrix: [0.0; 16],
            view_to_clip_matrix_prev: [0.0; 16],
            world_to_view_matrix: [0.0; 16],
            world_to_view_matrix_prev: [0.0; 16],
            world_prev_to_world_matrix: IDENTITY,
            motion_vector_scale: [1.0, 1.0, 0.0],
            camera_jitter: [0.0; 2],
            camera_jitter_prev: [0.0; 2],
            resolution_scale: [1.0, 1.0],
            resolution_scale_prev: [1.0, 1.0],
            time_delta_between_frames: 0.0,
            denoising_range: 500000.0,
            disocclusion_threshold: 0.01,
            disocclusion_threshold_alternate: 0.05,
            split_screen: 0.0,
            debug: 0.0,
            input_subrect_origin: [0, 0],
            frame_index: 0,
            accumulation_mode: AccumulationMode::Continue,
            is_motion_vector_in_world_space: false,
            is_history_confidence_available: false,
            is_disocclusion_threshold_mix_available: false,
            is_base_color_metalness_available: false,
            enable_validation: false,
        }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    //=============================================================================================================================
    // COMMON INPUTS
    //=============================================================================================================================

    // 3D world-space motion (RGBA16f+) or 2D screen-space motion (RG16f+), MVs must be non-jittered, MV = previous - current
    IN_MV,

    // Data must match encoding in "NRD_FrontEnd_PackNormalAndRoughness" and "NRD_FrontEnd_UnpackNormalAndRoughness" (RGBA8+)
    IN_NORMAL_ROUGHNESS,

    // Linear view depth for primary rays (R16f+)
    IN_VIEWZ,

    //=============================================================================================================================
    // INPUTS
    //=============================================================================================================================

    // Noisy radiance and hit distance (RGBA16f+)
    //      REBLUR: use "REBLUR_FrontEnd_PackRadianceAndNormHitDist" for encoding
    //      RELAX: use "RELAX_FrontEnd_PackRadianceAndHitDist" for encoding
    IN_DIFF_RADIANCE_HITDIST,
    IN_SPEC_RADIANCE_HITDIST,

    // Noisy hit distance (R8+)
    //      REBLUR: use "REBLUR_FrontEnd_GetNormHitDist" for encoding
    IN_DIFF_HITDIST,
    IN_SPEC_HITDIST,

    // Noisy bent normal and normalized hit distance (RGBA8+)
    //      REBLUR: use "REBLUR_FrontEnd_PackDirectionalOcclusion" for encoding
    IN_DIFF_DIRECTION_HITDIST,

    // Noisy SH data (2x RGBA16f+)
    //      REBLUR: use "REBLUR_FrontEnd_PackSh" for encoding
    //      RELAX: use "RELAX_FrontEnd_PackSh" for encoding
    IN_DIFF_SH0,
    IN_DIFF_SH1,
    IN_SPEC_SH0,
    IN_SPEC_SH1,

    // (Optional) User-provided history confidence in range 0-1, i.e. antilag (R8+)
    // Used only if "CommonSettings::isHistoryConfidenceAvailable = true"
    IN_DIFF_CONFIDENCE,
    IN_SPEC_CONFIDENCE,

    // (Optional) User-provided disocclusion threshold selector in range 0-1 (R8+)
    // Disocclusion threshold is mixed between "disocclusionThreshold" and "disocclusionThresholdAlternate"
    // Used only if "CommonSettings::isDisocclusionThresholdMixAvailable = true"
    IN_DISOCCLUSION_THRESHOLD_MIX,

    // (Optional) Base color (can be decoupled to diffuse and specular albedo based on metalness) and metalness (RGBA8+)
    // Used only if "CommonSettings::isBaseColorMetalnessAvailable = true"
    IN_BASECOLOR_METALNESS,

    // Noisy shadow data and optional translucency (RG16f+ and RGBA8+ for optional translucency)
    //      SIGMA: use "SIGMA_FrontEnd_PackShadow" for encoding
    IN_SHADOWDATA,
    IN_SHADOW_TRANSLUCENCY,

    // Noisy signal (R8+)
    IN_RADIANCE,

    // Primary and secondary world-space positions (RGBA16f+)
    IN_DELTA_PRIMARY_POS,
    IN_DELTA_SECONDARY_POS,

    //=============================================================================================================================
    // OUTPUTS
    //=============================================================================================================================

    // IMPORTANT: These textures can be potentially used as history buffers!
    // IMPORTANT: Most of denoisers do not write into output pixels outside of "CommonSettings::denoisingRange"!

    // Denoised radiance and hit distance
    //      REBLUR: use "REBLUR_BackEnd_UnpackRadianceAndNormHitDist" for decoding (RGBA16f+)
    //      RELAX: use "RELAX_BackEnd_UnpackRadiance" for decoding (R11G11B10f+)
    OUT_DIFF_RADIANCE_HITDIST,
    OUT_SPEC_RADIANCE_HITDIST,

    // Denoised SH data
    //      REBLUR: use "REBLUR_BackEnd_UnpackSh" for decoding (2x RGBA16f+)
    //      RELAX: use "RELAX_BackEnd_UnpackSh" for decoding (2x RGBA16f+)
    OUT_DIFF_SH0,
    OUT_DIFF_SH1,
    OUT_SPEC_SH0,
    OUT_SPEC_SH1,

    // Denoised normalized hit distance (R8+)
    OUT_DIFF_HITDIST,
    OUT_SPEC_HITDIST,

    // Denoised bent normal and normalized hit distance (RGBA8+)
    //      REBLUR: use "REBLUR_BackEnd_UnpackDirectionalOcclusion" for decoding
    OUT_DIFF_DIRECTION_HITDIST,

    // Denoised shadow and optional transcluceny (R8+ or RGBA8+)
    //      SIGMA: use "SIGMA_BackEnd_UnpackShadow" for decoding
    OUT_SHADOW_TRANSLUCENCY,

    // Denoised signal
    OUT_RADIANCE,

    // 2D screen-space specular motion (RG16f+), MV = previous - current
    OUT_REFLECTION_MV,

    // 2D screen-space refraction motion (RG16f+), MV = previous - current
    OUT_DELTA_MV,

    // (Optional) Debug output (RGBA8+), .w = transparency
    // Used if "CommonSettings::enableValidation = true"
    OUT_VALIDATION,

    //=============================================================================================================================
    // POOLS
    //=============================================================================================================================

    // Can be reused after denoising
    TRANSIENT_POOL,

    // Dedicated to NRD, can't be reused
    PERMANENT_POOL,
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceDesc {
    pub state_needed: DescriptorType,
    pub ty: ResourceType,
    pub index_in_pool: u16,
    pub mip_offset: u16,
    pub mip_num: u16,
}

#[repr(C)]
pub struct DispatchDesc {
    name: *const c_char,
    resources: *const ResourceDesc,
    resources_num: u32,
    constant_buffer_data: *const u8,
    constant_buffer_data_size: u32,
    pub pipeline_index: u16,
    pub grid_width: u16,
    pub grid_height: u16,
}
impl DispatchDesc {
    pub fn constant_buffer(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.constant_buffer_data,
                self.constant_buffer_data_size as usize,
            )
        }
    }
    pub fn resources(&self) -> &[ResourceDesc] {
        unsafe { std::slice::from_raw_parts(self.resources, self.resources_num as usize) }
    }
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name) }
    }
}

impl Debug for DispatchDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DispatchDesc")
            .field("name", &self.name())
            .field("resources", &self.resources())
            .field(
                "constant_buffer",
                &format!("{} bytes", self.constant_buffer().len()),
            )
            .field("pipeline_index", &self.pipeline_index)
            .field("grid_width", &self.grid_width)
            .field("grid_height", &self.grid_height)
            .finish()
    }
}

#[repr(C)]
pub struct HitDistanceParameters {
    // (units) - constant value
    // IMPORTANT: if your unit is not "meter", you must convert it from "meters" to "units" manually!
    pub a: f32,

    // (> 0) - viewZ based linear scale (1 m - 10 cm, 10 m - 1 m, 100 m - 10 m)
    pub b: f32,

    // (>= 1) - roughness based scale, use values > 1 to get bigger hit distance for low roughness
    pub c: f32,

    // (<= 0) - absolute value should be big enough to collapse "exp2( D * roughness ^ 2 )" to "~0" for roughness = 1
    pub d: f32,
}

impl Default for HitDistanceParameters {
    fn default() -> Self {
        Self {
            a: 3.0,
            b: 0.1,
            c: 20.0,
            d: -25.0,
        }
    }
}

// Antilag logic:
//    delta = ( abs( old - new ) - localVariance * sigmaScale ) / ( max( old, new ) + localVariance * sigmaScale + sensitivityToDarkness )
//    delta = LinearStep( thresholdMax, thresholdMin, delta )
//        - 1 - keep accumulation
//        - 0 - history reset
#[repr(C)]
pub struct AntilagIntensitySettings {
    // (normalized %) - must be big enough to almost ignore residual noise (boiling), default is tuned for 0.5rpp in general
    pub threshold_min: f32,

    // (normalized %) - max > min, usually 3-5x times greater than min
    pub threshold_max: f32,

    // (> 0) - real delta is reduced by local variance multiplied by this value
    pub sigma_scale: f32,

    // (intensity units) - bigger values make antilag less sensitive to lightness fluctuations in dark places
    pub sensitivity_to_darkness: f32, // IMPORTANT: 0 is a bad default

    // Ideally, must be enabled, but since "sensitivityToDarkness" requires fine tuning from the app side it is disabled by default
    pub enable: bool, // IMPORTANT: doesn't affect "occlusion" denoisers
}

impl Default for AntilagIntensitySettings {
    fn default() -> Self {
        Self {
            threshold_min: 0.3,
            threshold_max: 0.2,
            sigma_scale: 1.0,
            sensitivity_to_darkness: 0.0,
            enable: false,
        }
    }
}

#[repr(C)]
pub struct AntilagHitDistanceSettings {
    // (normalized %) - must almost ignore residual noise (boiling), default is tuned for 0.5rpp for the worst case
    pub threshold_min: f32,

    // (normalized %) - max > min, usually 2-4x times greater than min
    pub threshold_max: f32,

    // (> 0) - real delta is reduced by local variance multiplied by this value
    pub sigma_scale: f32,

    // (0; 1] - hit distances are normalized
    pub sensitivity_to_darkness: f32,

    // Enabled by default
    pub enable: bool,
}

impl Default for AntilagHitDistanceSettings {
    fn default() -> Self {
        Self {
            threshold_min: 0.03,
            threshold_max: 0.2,
            sigma_scale: 1.0,
            sensitivity_to_darkness: 0.1,
            enable: true,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum CheckerboardMode {
    Off,
    Black,
    White,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum HitDistanceReconstructionMode {
    // Probabilistic split at primary hit is not used, hence hit distance is always valid (reconstruction is not needed)
    Off,

    // If hit distance is invalid due to probabilistic sampling, reconstruct using 3x3 neighbors.
    // Probability at primary hit must be clamped to [1/4; 3/4] range to guarantee a sample in this area
    Area3x3,

    // If hit distance is invalid due to probabilistic sampling, reconstruct using 5x5 neighbors.
    // Probability at primary hit must be clamped to [1/16; 15/16] range to guarantee a sample in this area
    Area5x5,
}
#[repr(C)]
pub struct ReblurSettings {
    pub hit_distance_parameters: HitDistanceParameters,
    pub antilag_intensity_settings: AntilagIntensitySettings,
    pub antilag_hit_distance_settings: AntilagHitDistanceSettings,

    // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames (= FPS * "time of accumulation")
    pub max_accumulated_frame_num: u32,

    // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames in fast history (less than "maxAccumulatedFrameNum")
    pub max_fast_accumulated_frame_num: u32,

    // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - number of reconstructed frames after history reset (less than "maxFastAccumulatedFrameNum")
    pub history_fix_frame_num: u32,

    // (pixels) - pre-accumulation spatial reuse pass blur radius (0 = disabled, must be used in case of probabilistic sampling)
    pub diffuse_prepass_blur_radius: f32,
    pub specular_prepass_blur_radius: f32,

    // (pixels) - base denoising radius (30 is a baseline for 1440p)
    pub blur_radius: f32,

    // (pixels) - base stride between samples in history reconstruction pass
    pub history_fix_stride_between_samples: f32,

    // (normalized %) - base fraction of diffuse or specular lobe angle used to drive normal based rejection
    pub lobe_angle_fraction: f32,

    // (normalized %) - base fraction of center roughness used to drive roughness based rejection
    pub roughness_fraction: f32,

    // [0; 1] - if roughness < this, temporal accumulation becomes responsive and driven by roughness (useful for animated water)
    pub responsive_accumulation_roughness_threshold: f32,

    // (normalized %) - stabilizes output, more stabilization improves antilag (clean signals can use lower values)
    pub stabilization_strength: f32,

    // (normalized %) - represents maximum allowed deviation from local tangent plane
    pub plane_distance_sensitivity: f32,

    // IN_MV = lerp(IN_MV, specularMotion, smoothstep(specularProbabilityThresholdsForMvModification[0], specularProbabilityThresholdsForMvModification[1], specularProbability))
    pub specular_probability_thresholds_for_mv_modification: [f32; 2],

    // If not OFF and used for DIFFUSE_SPECULAR, defines diffuse orientation, specular orientation is the opposite
    pub checkerboard_mode: CheckerboardMode,

    // Must be used only in case of probabilistic sampling (not checkerboarding), when a pixel can be skipped and have "0" (invalid) hit distance
    pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

    // Adds bias in case of badly defined signals, but tries to fight with fireflies
    pub enable_anti_firefly: bool,

    // Turns off spatial filtering and virtual motion based specular tracking
    pub enable_reference_accumulation: bool,

    // Boosts performance by sacrificing IQ
    pub enable_performance_mode: bool,

    // Spatial passes do optional material index comparison as: ( materialEnabled ? material[ center ] == material[ sample ] : 1 )
    pub enable_material_test_for_diffuse: bool,
    pub enable_material_test_for_specular: bool,
}

impl Default for ReblurSettings {
    fn default() -> Self {
        Self {
            hit_distance_parameters: Default::default(),
            antilag_intensity_settings: Default::default(),
            antilag_hit_distance_settings: Default::default(),
            max_accumulated_frame_num: 30,
            max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_prepass_blur_radius: 30.0,
            specular_prepass_blur_radius: 50.0,
            blur_radius: 15.0,
            history_fix_stride_between_samples: 14.0,
            lobe_angle_fraction: 0.15,
            roughness_fraction: 0.15,
            responsive_accumulation_roughness_threshold: 0.0,
            stabilization_strength: 1.0,
            plane_distance_sensitivity: 0.005,
            specular_probability_thresholds_for_mv_modification: [0.5, 0.9],
            checkerboard_mode: CheckerboardMode::Off,
            hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
            enable_anti_firefly: false,
            enable_reference_accumulation: false,
            enable_performance_mode: false,
            enable_material_test_for_diffuse: false,
            enable_material_test_for_specular: false,
        }
    }
}

#[repr(C)]
pub struct SigmaSettings {
    // (normalized %) - represents maximum allowed deviation from local tangent plane
    pub plane_distance_sensitivity: f32,

    // [1; 3] - adds bias and stability if > 1
    pub blur_radius_scale: f32,
}
impl Default for SigmaSettings {
    fn default() -> Self {
        Self {
            plane_distance_sensitivity: 0.005,
            blur_radius_scale: 2.0,
        }
    }
}

// RELAX_DIFFUSE_SPECULAR
#[repr(C)]

pub struct RelaxDiffuseSpecularSettings {
    // (pixels) - pre-accumulation spatial reuse pass blur radius (0 = disabled, must be used in case of probabilistic sampling)
    pub diffuse_prepass_blur_radius: f32,
    pub specular_prepass_blur_radius: f32,

    // [0; RELAX_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames ( = FPS * "time of accumulation")
    pub diffuse_max_accumulated_frame_num: u32,
    pub specular_max_accumulated_frame_num: u32,

    // [0; RELAX_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames in fast history (less than "maxAccumulatedFrameNum")
    pub diffuse_max_fast_accumulated_frame_num: u32,
    pub specular_max_fast_accumulated_frame_num: u32,

    // [0; RELAX_MAX_HISTORY_FRAME_NUM] - number of reconstructed frames after history reset (less than "maxFastAccumulatedFrameNum")
    pub history_fix_frame_num: u32,

    // A-trous edge stopping Luminance sensitivity
    pub diffuse_phi_luminance: f32,
    pub specular_phi_luminance: f32,

    // (normalized %) - base fraction of diffuse or specular lobe angle used to drive normal based rejection
    pub diffuse_lobe_angle_fraction: f32,
    pub specular_lobe_angle_fraction: f32,

    // (normalized %) - base fraction of center roughness used to drive roughness based rejection
    pub roughness_fraction: f32,

    // (>= 0) - how much variance we inject to specular if reprojection confidence is low
    pub specular_variance_boost: f32,

    // (degrees) - slack for the specular lobe angle used in normal based rejection of specular during A-Trous passes
    pub specular_lobe_angle_slack: f32,

    // (pixels) - base stride between samples in history reconstruction pass
    pub history_fix_stride_between_samples: f32,

    // (> 0) - normal edge stopper for history reconstruction pass
    pub history_fix_edge_stopping_normal_power: f32,

    // [1; 3] - standard deviation scale of color box for clamping main "slow" history to responsive "fast" history
    pub history_clamping_color_box_sigma_scale: f32,

    // (>= 0) - history length threshold below which spatial variance estimation will be executed
    pub spatial_variance_estimation_history_threshold: u32,

    // [2; 8] - number of iteration for A-Trous wavelet transform
    pub atrous_iteration_num: u32,

    // [0; 1] - A-trous edge stopping Luminance weight minimum
    pub diffuse_min_luminance_weight: f32,
    pub specular_min_luminance_weight: f32,

    // (normalized %) - Depth threshold for spatial passes
    pub depth_threshold: f32,

    // Confidence inputs can affect spatial blurs, relaxing some weights in areas with low confidence
    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,

    // How much we relax roughness based rejection for spatial filter in areas where specular reprojection is low
    pub luminance_edge_stopping_relaxation: f32,
    pub normal_edge_stopping_relaxation: f32,

    // How much we relax rejection for spatial filter based on roughness and view vector
    pub roughness_edge_stopping_relaxation: f32,

    // If not OFF and used for DIFFUSE_SPECULAR, defines diffuse orientation, specular orientation is the opposite
    pub checkerboard_mode: CheckerboardMode,

    // Must be used only in case of probabilistic sampling (not checkerboarding), when a pixel can be skipped and have "0" (invalid) hit distance
    pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

    // Firefly suppression
    pub enable_anti_firefly: bool,

    // Skip reprojection test when there is no motion, might improve quality along the edges for static camera with a jitter
    pub enable_reprojection_test_skipping_without_motion: bool,

    // Roughness based rejection
    pub enable_roughness_edge_stopping: bool,

    // Spatial passes do optional material index comparison as: ( materialEnabled ? material[ center ] == material[ sample ] : 1 )
    pub enable_material_test_for_diffuse: bool,
    pub enable_material_test_for_specular: bool,
}

impl Default for RelaxDiffuseSpecularSettings {
    fn default() -> Self {
        Self {
            diffuse_prepass_blur_radius: 0.0,
            specular_prepass_blur_radius: 50.0,
            diffuse_max_accumulated_frame_num: 30,
            specular_max_accumulated_frame_num: 30,
            diffuse_max_fast_accumulated_frame_num: 6,
            specular_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_phi_luminance: 2.0,
            specular_phi_luminance: 1.0,
            diffuse_lobe_angle_fraction: 0.5,
            specular_lobe_angle_fraction: 0.5,
            roughness_fraction: 0.15,
            specular_variance_boost: 0.0,
            specular_lobe_angle_slack: 0.15,
            history_fix_stride_between_samples: 14.0,
            history_fix_edge_stopping_normal_power: 8.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            diffuse_min_luminance_weight: 0.0,
            specular_min_luminance_weight: 0.0,
            depth_threshold: 0.003,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            luminance_edge_stopping_relaxation: 0.5,
            normal_edge_stopping_relaxation: 0.3,
            roughness_edge_stopping_relaxation: 1.0,
            checkerboard_mode: CheckerboardMode::Off,
            hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
            enable_anti_firefly: false,
            enable_reprojection_test_skipping_without_motion: false,
            enable_roughness_edge_stopping: true,
            enable_material_test_for_diffuse: false,
            enable_material_test_for_specular: false,
        }
    }
}

// RELAX_DIFFUSE

#[repr(C)]
pub struct RelaxDiffuseSettings {
    pub prepass_blur_radius: f32,

    pub diffuse_max_accumulated_frame_num: u32,
    pub diffuse_max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,

    pub diffuse_phi_luminance: f32,
    pub diffuse_lobe_angle_fraction: f32,

    pub history_fix_edge_stopping_normal_power: f32,
    pub history_fix_stride_between_samples: f32,

    pub history_clamping_color_box_sigma_scale: f32,

    pub spatial_variance_estimation_history_threshold: u32,
    pub atrous_iteration_num: u32,
    pub min_luminance_weight: f32,
    pub depth_threshold: f32,

    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,

    pub checkerboard_mode: CheckerboardMode,
    pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

    pub enable_anti_firefly: bool,
    pub enable_reprojection_test_skipping_without_motion: bool,
    pub enable_material_test: bool,
}

impl Default for RelaxDiffuseSettings {
    fn default() -> Self {
        Self {
            prepass_blur_radius: 0.0,
            diffuse_max_accumulated_frame_num: 30,
            diffuse_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_phi_luminance: 2.0,
            diffuse_lobe_angle_fraction: 0.5,
            history_fix_edge_stopping_normal_power: 8.0,
            history_fix_stride_between_samples: 14.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            min_luminance_weight: 0.0,
            depth_threshold: 0.01,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            checkerboard_mode: CheckerboardMode::Off,
            hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
            enable_anti_firefly: false,
            enable_reprojection_test_skipping_without_motion: false,
            enable_material_test: false,
        }
    }
}

// RELAX_SPECULAR

#[repr(C)]
pub struct RelaxSpecularSettings {
    pub prepass_blur_radius: f32,

    pub specular_max_accumulated_frame_num: u32,
    pub specular_max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,

    pub specular_phi_luminance: f32,
    pub diffuse_lobe_angle_fraction: f32,
    pub specular_lobe_angle_fraction: f32,
    pub roughness_fraction: f32,

    pub specular_variance_boost: f32,
    pub specular_lobe_angle_slack: f32,

    pub history_fix_edge_stopping_normal_power: f32,
    pub history_fix_stride_between_samples: f32,

    pub history_clamping_color_box_sigma_scale: f32,

    pub spatial_variance_estimation_history_threshold: u32,
    pub atrous_iteration_num: u32,
    pub min_luminance_weight: f32,
    pub depth_threshold: f32,

    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,

    pub luminance_edge_stopping_relaxation: f32,
    pub normal_edge_stopping_relaxation: f32,
    pub roughness_edge_stopping_relaxation: f32,

    pub checkerboard_mode: CheckerboardMode,
    pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

    pub enable_anti_firefly: bool,
    pub enable_reprojection_test_skipping_without_motion: bool,
    pub enable_roughness_edge_stopping: bool,
    pub enable_material_test: bool,
}

impl Default for RelaxSpecularSettings {
    fn default() -> Self {
        Self {
            prepass_blur_radius: 50.0,
            specular_max_accumulated_frame_num: 30,
            specular_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            specular_phi_luminance: 1.0,
            diffuse_lobe_angle_fraction: 0.5,
            specular_lobe_angle_fraction: 0.5,
            roughness_fraction: 0.15,
            specular_variance_boost: 0.0,
            specular_lobe_angle_slack: 0.15,
            history_fix_edge_stopping_normal_power: 8.0,
            history_fix_stride_between_samples: 14.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            min_luminance_weight: 0.0,
            depth_threshold: 0.01,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            luminance_edge_stopping_relaxation: 0.5,
            normal_edge_stopping_relaxation: 0.3,
            roughness_edge_stopping_relaxation: 1.0,
            checkerboard_mode: CheckerboardMode::Off,
            hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
            enable_anti_firefly: false,
            enable_reprojection_test_skipping_without_motion: false,
            enable_roughness_edge_stopping: true,
            enable_material_test: false,
        }
    }
}

pub struct ReferenceSettings {
    // (>= 0) - maximum number of linearly accumulated frames ( = FPS * "time of accumulation")
    pub max_accumulated_frame_num: u32,
}
impl Default for ReferenceSettings {
    fn default() -> Self {
        Self {
            max_accumulated_frame_num: 1024,
        }
    }
}

#[allow(non_snake_case)]
extern "fastcall" {
    pub(crate) fn GetLibraryDesc() -> &'static LibraryDesc;
    pub(crate) fn CreateInstance(desc: &InstanceCreationDesc, instance: &mut *mut c_void)
        -> Result;
    pub(crate) fn DestroyInstance(instance: *mut c_void);
    pub(crate) fn GetInstanceDesc(instance: *mut c_void) -> *const InstanceDesc;
    pub(crate) fn SetCommonSettings(instance: *mut c_void, settings: &CommonSettings) -> Result;
    pub(crate) fn SetDenoiserSettings(
        instance: *mut c_void,
        identifier: Identifier,
        denoiserSettings: *const c_void,
    ) -> Result;
    pub(crate) fn GetComputeDispatches(
        instance: *mut c_void,
        identifiers: *const Identifier,
        identifiers_num: u32,
        descs: &mut *const DispatchDesc,
        descs_num: &mut u32,
    ) -> Result;
}
