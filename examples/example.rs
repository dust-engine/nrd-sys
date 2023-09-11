fn main() {
    let lib_desc = nrd_sys::Instance::library_desc();
    println!("{:#?}", lib_desc);
    let id1 = nrd_sys::Identifier(0);
    let mut instance = nrd_sys::Instance::new(&[nrd_sys::DenoiserDesc {
        identifier: id1,
        denoiser: nrd_sys::Denoiser::ReblurDiffuse,
        render_width: 100,
        render_height: 100,
    }])
    .unwrap();
    let desc = instance.desc();
    println!("{:#?}", desc);

    instance
        .set_common_settings(&nrd_sys::CommonSettings::default())
        .unwrap();
    instance
        .set_denoiser_settings(id1, &nrd_sys::ReferenceSettings::default())
        .unwrap();

    let dispatches = instance.get_compute_dispatches(&[id1]).unwrap();

    println!("{:#?}", dispatches);
}
