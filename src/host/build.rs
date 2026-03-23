fn main() {
    // Bypassed for host linting to avoid succinct rustc toolchain flag incompatibilities.
    // sp1_build::build_program("../guest");
    println!("cargo:warning=Skipping SP1 guest build. Use the SP1 CLI or set up the environment completely to build the guest ELF.");
}
