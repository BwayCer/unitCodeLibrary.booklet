use std::process;

use clap::Parser;

#[allow(unused_imports)]
use ffi_test;

#[derive(Parser, Debug)]
#[command(name = "generate_headers")]
struct InputArgs {
    #[arg(
        short,
        long = "file",
        default_value = "",
        help = "Set a path name for generated headers (default: \"LinkLib.h\" when lang = \"c\"; \"LinkLib.g.cs\" when lang = \"cs\")"
    )]
    file_path: String,

    #[arg(
        short,
        long,
        default_value = "c",
        help = "Select language for generating headers (ex: \"c\", \"cs\")"
    )]
    lang: String,
}

fn main() {
    let args = &InputArgs::parse();

    let (file_path, lang) = match args.lang.as_str() {
        "c" => ("LinkLib.h", Language::C),
        "cs" => ("LinkLib.g.cs", Language::CSharp),
        _ => {
            eprintln!(
                "Error: Please provide the language to generate headers. (ex: \"c\", \"cs\")"
            );
            process::exit(1);
        }
    };

    let file_path = if args.file_path.is_empty() {
        file_path
    } else {
        args.file_path.as_str()
    };

    #[cfg(feature = "headers")]
    generate_headers(file_path, lang).expect(&format!("生成 {} 文件失敗", file_path));
}

enum Language {
    C,
    CSharp,
}

#[cfg(feature = "headers")]
fn generate_headers(file_path: &str, lang: Language) -> ::std::io::Result<()> {
    let lang_ = match lang {
        Language::C => ::safer_ffi::headers::Language::C,
        Language::CSharp => ::safer_ffi::headers::Language::CSharp,
    };
    ::safer_ffi::headers::builder()
        .with_language(lang_)
        .to_file(file_path)?
        .generate()
}
