fn main() {
    let platform = config::PLATFORM;
    println!("cargo:rustc-cfg=platform=\"{}\"", platform);
    println!("cargo:rustc-cfg=platform_family=\"{}\"", config::FAMILY);
}
