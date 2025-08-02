use std::{env, fs};
use std::path::{Path, PathBuf};
use clap::Parser;
use spirv_builder::{ShaderPanicStrategy, SpirvBuilder};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the shader crate to build
    #[clap(short, long, default_value = ".")]
    source: PathBuf,
    /// Directory to write the compiled shader to
    #[clap(short, long)]
    destination: Option<PathBuf>,
    /// The platform to target during compilation (see: https://rust-gpu.github.io/rust-gpu/book/platform-support.html)
    #[clap(short, long, default_value = "spirv-unknown-vulkan1.4")]
    target: String,
    /// Spir-V module extension to enable for the shader
    #[clap(short, long)]
    extension: Vec<String>,
    /// Spir-V module capability to enable for the shader
    #[clap(short, long)]
    capability: Vec<String>,
    /// Enable debug printf statements
    #[clap(long)]
    debug: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_home = env!("CARGO_HOME");
    unsafe {
        env::set_var("LD_LIBRARY_PATH", Path::new(cargo_home).join("lib"));
    }
    let args = Args::parse();
    let current_dir = env::current_dir()?;
    let source_path = current_dir.join(args.source);

    let mut builder = SpirvBuilder::new(source_path, args.target);
    builder.toolchain_overwrite = Some("nightly-2025-06-23".into());
    if args.debug {
        builder.shader_panic_strategy = ShaderPanicStrategy::DebugPrintfThenExit {
            print_inputs: true,
            print_backtrace: true
        }
    }
    builder.extensions = args.extension;
    for capability in args.capability {
        if let Ok(capability) = capability.parse() {
            builder.capabilities.push(capability)
        }
    }
    let result = builder.build()?;

    if let Some(destination) = args.destination {
        let artifact_path = result.module.unwrap_single();
        let artifact_name = artifact_path.file_name().unwrap();
        let artifact = fs::read(&artifact_path)?;
        fs::write(current_dir.join(destination).join(artifact_name), artifact)?;
    }
    Ok(())
}
