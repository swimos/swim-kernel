use std::env;

fn main() {
    let mode = env::var("JVM_LINK").ok();

    if let Some(mode) = mode {
        let lib_dir = env::var("JVM_LIB_DIR").or_else(|_| {
            env::var("JAVA_HOME").map(|java_home| {
                //java_home + "/jre/lib/server" // Java version < 9
                java_home + "/lib/server" // Java version >= 9
            })
        }).ok();

        let libs_env = env::var("JVM_LIBS").ok();
        let libs = match libs_env {
            Some(ref v) => v.split(":").collect(),
            #[cfg(not(target_os = "macos"))]
            None => vec!["jvm"],
            #[cfg(target_os = "macos")]
            None => vec!["jvm", "jli"],
        };

        if let Some(lib_dir) = lib_dir {
            println!("cargo:rustc-link-search=native={}", lib_dir);
            if libs.contains(&"jli") {
                println!("cargo:rustc-link-search=native={}", lib_dir + "/../jli");
            }
        }

        for lib in libs {
            println!("cargo:rustc-link-lib={}={}", mode, lib);
        }
    }
}
