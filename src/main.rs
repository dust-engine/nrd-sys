fn main() {
    let id1 = nrd::Identifier(0);
    let mut instance = nrd::Instance::new(&[nrd::DenoiserDesc {
        identifier: id1,
        denoiser: nrd::Denoiser::Reference,
        render_width: 100,
        render_height: 100,
    }])
    .unwrap();
    let desc = instance.desc();
    println!("{:#?}", desc);

    instance
        .set_common_settings(&nrd::CommonSettings::default())
        .unwrap();
    instance
        .set_denoiser_settings(id1, &nrd::ReferenceSettings::default())
        .unwrap();

    let dispatches = instance.get_compute_dispatches(&[id1]).unwrap();

    println!("{:#?}", dispatches);
}
