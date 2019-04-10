extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::process::Command;

/// Name of Rust cfg variable that will hold Python configuration flags.
const CFG_KEY: &'static str = "py_sys_config";

/// C preprocessor that will map to cargo `--cfg=py_sys_config={key}`.
#[cfg(not(target_os="windows"))]
static SYSCONFIG_FLAGS: [&'static str; 7] = [
    "Py_USING_UNICODE",
    "Py_UNICODE_WIDE",
    "WITH_THREAD",
    "Py_DEBUG",
    "Py_REF_DEBUG",
    "Py_TRACE_REFS",
    "COUNT_ALLOCS",
];

/// C preprocessor defines that will map to cargo `--cfg=py_sys_config={key}_{val}`
static SYSCONFIG_PROPS: [&'static str; 1] = [
    "Py_UNICODE_SIZE",
];
fn is_sysconfig_prop(key: &str) -> bool {
    SYSCONFIG_PROPS.iter().find(|k| **k == key).is_some()
}

#[cfg(not(target_os="windows"))]
static NEWLINE_SEQUENCE: &'static str = "\n";
#[cfg(target_os="windows")]
static NEWLINE_SEQUENCE: &'static str = "\r\n";

struct PythonVersion {
    major: u8,
    minor: Option<u8>,
}
impl PythonVersion {
    fn matches(&self, that: &PythonVersion) -> bool {
        self.major == that.major && (self.minor.is_none() || self.minor == that.minor)
    }
}
impl fmt::Display for PythonVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.major.fmt(f)?;
        f.write_str(".")?;
        match self.minor {
            Some(minor) => minor.fmt(f),
            None => f.write_str("*"),
        }
    }
}

struct PythonConfig {
    version: PythonVersion,
    executable: String,
    exec_prefix: String,
    lib_dir: String,
    ld_version: String,
    enable_shared: bool,
}

/// Returns the target python version specified by cargo feature flags.
fn get_python_feature_version() -> PythonVersion {
    let feature_regex = Regex::new(r"CARGO_FEATURE_PYTHON_(\d+)(_(\d+))?").unwrap();
    let mut env_vars = env::vars().collect::<Vec<_>>();
    env_vars.sort_by(|a, b| b.cmp(a)); // Sort by reverse lexical order.
    for (key, _) in env_vars {
        if let Some(capture) = feature_regex.captures(&key) {
            return PythonVersion {
                major: capture.get(1).unwrap().as_str().parse().unwrap(),
                minor: match capture.get(3) {
                    Some(s) => Some(s.as_str().parse().unwrap()),
                    None => None
                }
            };
        }
    }
    panic!("Python version feature not found");
}

/// Executes a python `script` using `interpreter`, returning stdout on success,
/// and stderr on failure.
fn run_python_script(interpreter: &str, script: &str) -> Result<String, String> {
    let mut cmd = Command::new(interpreter);
    cmd.arg("-c").arg(script);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(error) => return Err(format!("Failed to run python interpreter `{:?}`: {}", cmd, error)),
    };

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        let mut msg = format!("Failed to run python script:\n\n");
        msg.push_str(&stderr);
        Err(msg)
    }
}

/// Invokes a python `interpreter`, and returns its configuration.
fn interpret_python_config(interpreter: &str) -> PythonConfig {
    let script = "import sys; import sysconfig; print(sys.executable); \
print(sys.version_info[0:2]); \
print(sys.exec_prefix); \
print(sysconfig.get_config_var('LIBDIR')); \
print(sysconfig.get_config_var('LDVERSION') or '%s%s' % (sysconfig.get_config_var('py_version_short'), sysconfig.get_config_var('DEBUG_EXT') or '')); \
print(sysconfig.get_config_var('Py_ENABLE_SHARED'));";
    let output = run_python_script(interpreter, script).unwrap();

    let mut lines: Vec<String> = output.split(NEWLINE_SEQUENCE).map(|line| line.to_owned()).collect();
    let executable = lines.remove(0);
    let version = lines.remove(0);
    let version = match Regex::new(r"\((\d+), (\d+)\)").unwrap().captures(&version) {
        Some(capture) => PythonVersion {
            major: capture.get(1).unwrap().as_str().parse().unwrap(),
            minor: Some(capture.get(2).unwrap().as_str().parse().unwrap()),
        },
        None => panic!("Malformed python version {}", version),
    };
    let exec_prefix = lines.remove(0);
    let lib_dir = lines.remove(0);
    let ld_version = lines.remove(0);
    let enable_shared = lines.remove(0) == "1";

    PythonConfig {
        version: version,
        executable: executable,
        exec_prefix: exec_prefix,
        lib_dir: lib_dir,
        ld_version: ld_version,
        enable_shared: enable_shared,
    }
}

/// Searches the environment for a matching python `version`, and returns its configuration.
fn resolve_python_config(version: &PythonVersion) -> PythonConfig {
    if let Some(sys_executable) = env::var_os("PYTHON_SYS_EXECUTABLE") {
        // Examine `$PYTHON_SYS_EXECUTABLE` executable.
        let interpreter_path = sys_executable.to_str().unwrap();
        let config = interpret_python_config(interpreter_path);
        if version.matches(&config.version) {
            return config;
        } else {
            panic!("Incompatible python version in PYTHON_SYS_EXECUTABLE={}; expected {}, but found {}",
                   config.executable, version, config.version);
        }
    }

    // Examine `python` executable.
    let config = interpret_python_config("python");
    if version.matches(&config.version) {
        return config;
    }

    // Examine `python{major}` executable.
    let interpreter_path = &format!("python{}", version.major);
    let config = interpret_python_config(interpreter_path);
    if version.matches(&config.version) {
        return config;
    }

    if let Some(minor) = version.minor {
        // Examime `python{major}.{minor} executable.
        let interpreter_path = &format!("python{}.{}", version.major, minor);
        let config = interpret_python_config(interpreter_path);
        if version.matches(&config.version) {
            return config;
        }
    }

    panic!("No python interpreter found for version {}", version);
}

/// Prints rustc configuration parameters to stdout.
fn configure_python_library(config: &PythonConfig) {
    let is_python_module = env::var_os("CARGO_FEATURE_PYTHON_MODULE").is_some();
    if !is_python_module || cfg!(target_os="windows") {
        // Configure library link flags.
        if cfg!(target_os="macos") {
            let script = "import sysconfig; print('framework' if sysconfig.get_config_var('PYTHONFRAMEWORK') else ('shared' if sysconfig.get_config_var('Py_ENABLE_SHARED') else 'static'));";
            let output = run_python_script("python", script).unwrap();
            let link_model = output.trim_end();
            match link_model {
                "static" => println!("cargo:rustc-link-lib=static=python{}", config.ld_version),
                "shared" | "framework" => println!("cargo:rustc-link-lib=python{}", config.ld_version),
                _ => panic!("Unknown link model: {}", link_model),
            }
        } else if cfg!(target_os="windows") {
            if let Some(minor) = config.version.minor {
                println!("cargo:rustc-link-lib=python{}{}", config.version.major, minor);
            } else {
                println!("cargo:rustc-link-lib=python{}", config.version.major);
            }
        } else if config.enable_shared {
            println!("cargo:rustc-link-lib=python{}", config.ld_version);
        } else {
            println!("cargo:rustc-link-lib=static=python{}", config.ld_version);
        }

        // Configure library search path.
        if config.lib_dir != "None" {
            println!("cargo:rustc-link-search=native={}", config.lib_dir);
        } else if cfg!(target_os="windows") {
            println!("cargo:rustc-link-search=native={}\\libs", config.exec_prefix);
        }
    }

    // Configure library version flags.
    if let PythonVersion { major: 3, minor } = config.version {
        if env::var_os("CARGO_FEATURE_PEP_384").is_some() {
            println!("cargo:rustc-cfg=Py_LIMITED_API");
        }
        if let Some(minor) = minor {
            for i in 4..(minor+1) {
                println!("cargo:rustc-cfg=Py_3_{}", i);
            }
        }
    }
}

#[cfg(not(target_os="windows"))]
fn get_python_sys_config(interpreter: &String) -> HashMap<&'static str, String> {
    let mut script = "import sysconfig;".to_owned();
    script.push_str("config = sysconfig.get_config_vars();");

    for key in SYSCONFIG_FLAGS.iter() {
        script.push_str(&format!("print(config.get('{}', 0));", key));
    }
    for key in SYSCONFIG_PROPS.iter() {
        script.push_str(&format!("print(config.get('{}', None));", key));
    }

    let mut cmd = Command::new(interpreter);
    cmd.arg("-c").arg(script);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(error) => panic!("Failed to run python interpreter `{:?}`: {}", cmd, error),
    };

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let lines: Vec<&str> = stdout.trim_end().split(NEWLINE_SEQUENCE).collect();
        let mut sys_config = HashMap::<&'static str, String>::new();
        let mut vals = lines.iter();
        for &key in SYSCONFIG_FLAGS.iter() {
            let val = *vals.next().unwrap();
            if val != "0" {
                sys_config.insert(key, val.to_owned());
            }
        }
        for &key in SYSCONFIG_PROPS.iter() {
            let val = *vals.next().unwrap();
            if val != "None" {
                sys_config.insert(key, val.to_owned());
            }
        }
        assert!(vals.next().is_none());
        sys_config
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        let mut msg = format!("Failed to run python script:\n\n");
        msg.push_str(&stderr);
        panic!(msg);
    }
}
#[cfg(target_os="windows")]
fn get_python_sys_config(_interpreter: &String) -> HashMap<&'static str, String> {
    // Hardcode sys_config on Windows.
    let mut sys_config = HashMap::<String, String>::new();
    sys_config.insert("Py_USING_UNICODE", "1".to_owned());
    sys_config.insert("Py_UNICODE_WIDE", "0".to_owned());
    sys_config.insert("WITH_THREAD".to_owned(), "1".to_owned());
    //sys_config.insert("Py_DEBUG", "1");
    //sys_config.insert("Py_REF_DEBUG", "1");
    //sys_config.insert("Py_TRACE_REFS", "1");
    //sys_config.insert("COUNT_ALLOCS", 1");
    sys_config.insert("Py_UNICODE_SIZE", "2".to_owned());
    sys_config
}

fn main() {
    let version = get_python_feature_version();
    let config = resolve_python_config(&version);
    configure_python_library(&config);

    let mut sys_config = get_python_sys_config(&config.executable);
    if sys_config.get("Py_DEBUG").is_some() {
        sys_config.insert("Py_TRACE_REFS", "1".to_owned()); // Py_DEBUG implies Py_TRACE_REFS.
    }
    if sys_config.get("Py_TRACE_REFS").is_some() {
        sys_config.insert("Py_REF_DEBUG", "1".to_owned()); // Py_TRACE_REFS implies Py_REF_DEBUG.
    }

    let mut flags = String::new();
    for (key, val) in &sys_config {
        if !flags.is_empty() {
            flags.push(',');
        }
        if is_sysconfig_prop(key) {
            flags.push_str(format!("PROP_{}={}", key, val).as_ref());
            println!("cargo:rustc-cfg={}=\"{}_{}\"", CFG_KEY, key, val);
        } else {
            flags.push_str(format!("FLAG_{}={}", key, val).as_ref());
            println!("cargo:rustc-cfg={}=\"{}\"", CFG_KEY, key);
        }
    }

    println!("cargo:python_flags={}", flags);
    println!("cargo:python_interpreter={}", config.executable);
}
