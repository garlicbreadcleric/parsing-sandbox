fn main() {
  let mut simdutf_wrapper_config = cmake::Config::new("libsimdutf-wrapper");
  simdutf_wrapper_config.profile("Release");
  let simdutf_wrapper_dir = simdutf_wrapper_config.build();

  println!("cargo:rustc-link-search=native={}/build", simdutf_wrapper_dir.display());
  println!("cargo:rustc-link-lib=dylib=simdutf_wrapper");
}
