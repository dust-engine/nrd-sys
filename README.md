# nrd-sys
Rust bindings for the [NVIDIA Real Time Denoisers SDK](https://github.com/NVIDIAGameWorks/RayTracingDenoiser), targeting Vulkan applications.

> NVIDIA Real-Time Denoisers (NRD) is a spatio-temporal API agnostic denoising library. The library has been designed to work with low rpp (ray per pixel) signals. NRD is a fast solution that slightly depends on input signals and environment conditions.


Supports both Windows and Linux. Pre-compiled binaries are automatically downloaded and linked.

The binaries are compiled from a custom NRD [fork](https://github.com/dust-engine/NVIDIA_RayTracingDenoiser), which is
necessary to make the SDK work better with Rust. The fork contains the following changes:

- CI to compile the binaries.
- Updated the allocator interface so that the allocation size and alignments (as required by the Rust allocation API) are passed in on `free`.


## Usage
```rs
fn main() {
    let lib_desc = nrd_sys::Instance::library_desc();

    let id1 = nrd_sys::Identifier(0);
    let mut instance = nrd_sys::Instance::new(&[nrd_sys::DenoiserDesc {
        identifier: id1,
        denoiser: nrd_sys::Denoiser::ReblurDiffuse,
        render_width: 100,
        render_height: 100,
    }])
    .unwrap();
    let desc = instance.desc();

    instance
        .set_common_settings(&nrd_sys::CommonSettings::default())
        .unwrap();
    instance
        .set_denoiser_settings(id1, &nrd_sys::ReferenceSettings::default())
        .unwrap();

    let dispatches = instance.get_compute_dispatches(&[id1]).unwrap();

    // Now, make API calls based on `dispatches`
}

```

## Note
The pre-compiled binaries included with this package only contains SPIR-V shader. If you need DXIL / DXBC shader for DirectX integration, feel free to make that a Cargo feature and create a PR. 
