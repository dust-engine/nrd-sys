#![feature(allocator_api)]

use std::ffi::c_void;

mod ffi;

pub use ffi::{
    CheckerboardMode, CommonSettings, Denoiser, DenoiserDesc, DispatchDesc,
    HitDistanceReconstructionMode, Identifier, ReferenceSettings, ResourceType,
    DescriptorType, Sampler, ReblurSettings, RelaxDiffuseSettings,
    RelaxDiffuseSpecularSettings, RelaxSpecularSettings, SigmaSettings,
    Format, TextureDesc, ResourceDesc, SPIRVBindingOffsets
};

mod allocator {
    use std::alloc::{Allocator, Layout};
    use std::ffi::c_void;
    use std::ptr::NonNull;
    pub extern "C" fn allocate(
        _user_arg: *const c_void,
        size: usize,
        alignment: usize,
    ) -> *mut c_void {
        unsafe {
            match std::alloc::Global.allocate(Layout::from_size_align_unchecked(size, alignment)) {
                Ok(ptr) => ptr.as_ptr() as *mut c_void,
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
    pub extern "C" fn reallocate(
        user_arg: *const c_void,
        memory: *mut c_void,
        old_size: usize,
        old_alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut c_void {
        free(user_arg, memory, old_size, old_alignment);
        allocate(user_arg, new_size, new_alignment)
    }
    pub extern "C" fn free(
        _user_arg: *const c_void,
        memory: *mut c_void,
        size: usize,
        alignment: usize,
    ) {
        let memory = NonNull::new(memory).unwrap();
        unsafe {
            std::alloc::Global.deallocate(
                memory.cast(),
                Layout::from_size_align_unchecked(size, alignment),
            )
        }
    }
}

pub trait DenoiserSettings {}
impl DenoiserSettings for ffi::ReblurSettings {}
impl DenoiserSettings for ffi::RelaxDiffuseSettings {}
impl DenoiserSettings for ffi::RelaxDiffuseSpecularSettings {}
impl DenoiserSettings for ffi::RelaxSpecularSettings {}
impl DenoiserSettings for ffi::ReferenceSettings {}
impl DenoiserSettings for ffi::SigmaSettings {}

pub struct Instance(*mut c_void);
impl Instance {
    pub fn library_desc() -> &'static ffi::LibraryDesc {
        unsafe { ffi::GetLibraryDesc() }
    }
    pub fn new(denoisers: &[ffi::DenoiserDesc]) -> Result<Self, ffi::Result> {
        let desc = ffi::InstanceCreationDesc {
            memory_allocator_interface: ffi::MemoryAllocatorInterface {
                allocate: allocator::allocate,
                reallocate: allocator::reallocate,
                free: allocator::free,
                user_arg: std::ptr::null(),
            },
            denoisers: denoisers.as_ptr(),
            denoisers_num: denoisers.len() as u32,
        };
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let result = unsafe { ffi::CreateInstance(&desc, &mut ptr) };
        match result {
            ffi::Result::Success => Ok(Self(ptr)),
            _ => Err(result),
        }
    }

    pub fn desc(&self) -> &ffi::InstanceDesc {
        unsafe {
            let ptr = ffi::GetInstanceDesc(self.0);
            &*ptr
        }
    }

    pub fn set_common_settings(
        &mut self,
        settings: &ffi::CommonSettings,
    ) -> Result<(), ffi::Result> {
        unsafe {
            let result = ffi::SetCommonSettings(self.0, settings);
            match result {
                ffi::Result::Success => Ok(()),
                _ => Err(result),
            }
        }
    }
    pub fn set_denoiser_settings<T>(
        &mut self,
        identifier: ffi::Identifier,
        reblur_settings: &T,
    ) -> Result<(), ffi::Result> {
        unsafe {
            ffi::SetDenoiserSettings(
                self.0,
                identifier,
                reblur_settings as *const _ as *const c_void,
            )
            .ok(())
        }
    }

    pub fn get_compute_dispatches(
        &mut self,
        identifiers: &[ffi::Identifier],
    ) -> Result<&[ffi::DispatchDesc], ffi::Result> {
        unsafe {
            let mut dispatches: *const ffi::DispatchDesc = std::ptr::null();
            let mut dispatches_count: u32 = 0;
            let result = ffi::GetComputeDispatches(
                self.0,
                identifiers.as_ptr(),
                identifiers.len() as u32,
                &mut dispatches,
                &mut dispatches_count,
            );
            match result {
                ffi::Result::Success => Ok(std::slice::from_raw_parts(
                    dispatches,
                    dispatches_count as usize,
                )),
                _ => Err(result),
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            ffi::DestroyInstance(self.0);
        }
    }
}
