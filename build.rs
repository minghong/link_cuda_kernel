extern crate cc;

fn main() {
    // cc::Build::new()
    //     .cuda(true)
    //     .flag("-cudart=shared")
    //     .flag("-gencode")
    //     .flag("arch=compute_61,code=sm_61")
    //     .file("kernel.cu")
    //     .compile("libvector_add.a");
<<<<<<< HEAD
=======

    println!("cargo:rustc-link-search=native=/home/englefly/work/link_cuda_kernel/");
    println!("cargo:rustc-link-lib=static=vector_add");
    println!("cargo:rustc-link-lib=stdc++");
>>>>>>> b3cf00430f00465df388dd7a8c67e2a685b228aa

    /* Link CUDA Runtime (libcudart.so) */

    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=vector_add");
    println!("cargo:rustc-link-lib=stdc++");


    // Add link directory
    // - This path depends on where you install CUDA (i.e. depends on your Linux distribution)
    // - This should be set by `$LIBRARY_PATH`
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");

    /* Optional: Link CUDA Driver API (libcuda.so) */

    // println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64/stub");
    // println!("cargo:rustc-link-lib=cuda");
}
