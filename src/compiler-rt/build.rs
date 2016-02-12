extern crate cmake;
extern crate gcc;

use std::{env, fs};
use std::path::PathBuf;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

// FIXME(japaric) duplicate function in bootstrap::build::util
fn staticlib(name: &str, target: &str) -> String {
    if target.contains("windows-msvc") {
        format!("{}.lib", name)
    } else {
        format!("lib{}.a", name)
    }
}

fn main() {
    if let Ok(stage) = env::var("RUSTC_STAGE") {
        match &stage[..] {
            "0" => {},
            // Nothing to do build for stage 1 onwards
            _ => return,
        }
    }

    let ref build_llvm_config = env::var_os("COMPILER_RT_LLVM_CONFIG").unwrap();
    let ref cc = env::var_os("COMPILER_RT_C_COMPILER").unwrap();
    let ref out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ref profile = env::var("COMPILER_RT_PROFILE").unwrap();

    let ref target = env::var("TARGET").unwrap();
    let ref build = env::var("HOST").unwrap();

    let arch = target.split('-').next().unwrap();
    let (dir, build_target, libname) = if target.contains("linux") {
        let os = if target.contains("android") {"-android"} else {""};
        let target = format!("clang_rt.builtins-{}{}", arch, os);
        ("linux".to_string(), target.clone(), target)
    } else if target.contains("darwin") {
        let target = format!("clang_rt.builtins_{}_osx", arch);
        ("builtins".to_string(), target.clone(), target)
    } else if target.contains("windows-gnu") {
        let target = format!("clang_rt.builtins-{}", arch);
        ("windows".to_string(), target.clone(), target)
    } else if target.contains("windows-msvc") {
        (format!("windows/{}", profile),
         "lib/builtins/builtins".to_string(),
         format!("clang_rt.builtins-{}", arch.replace("i686", "i386")))
    } else {
        panic!("can't get os from target: {}", target)
    };

    let _ = fs::remove_dir_all(out_dir);
    t!(fs::create_dir_all(out_dir));

    let mut cfg = cmake::Config::new("src");
    cfg.target(target)
       .host(build)
       .out_dir(out_dir)
       .profile(profile)
       .define("LLVM_CONFIG_PATH", build_llvm_config)
       .define("COMPILER_RT_DEFAULT_TARGET_TRIPLE", target)
       .define("COMPILER_RT_BUILD_SANITIZERS", "OFF")
       .define("COMPILER_RT_BUILD_EMUTLS", "OFF")
       .define("CMAKE_C_COMPILER", cc)
       .build_target(&build_target);
    cfg.build();

    // copy the static library to a more arch-independent place
    let src = out_dir.join(format!("build/lib/{}/{}", dir, staticlib(&libname, target)));
    let ref rtlib = out_dir.join("build/lib/libcompiler-rt.a");
    t!(fs::hard_link(src, rtlib));

    println!("cargo:rustc-link-search=native={}", out_dir.join("build/lib").display());

    // Copy build artifacts where `bootstrap` expects them
    if let Some(out_dir) = env::var_os("COMPILER_RT_OUT_DIR") {
        t!(fs::create_dir_all(&out_dir));

        let mut dst = PathBuf::from(out_dir);
        dst.push("libcompiler-rt.a");

        t!(fs::hard_link(rtlib, dst));
    }
}
