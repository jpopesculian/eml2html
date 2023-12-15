use clap::Parser;
use eml_parser::EmlParser;
use eyre::bail;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let parsed = EmlParser::from_file(args.filename)?.with_body().parse()?;
    let content_transfer_encoding = parsed
        .headers
        .iter()
        .find(|h| h.name == "Content-Transfer-Encoding")
        .map(|h| h.value.to_string());
    let body = parsed.body.as_deref().unwrap_or("");
    match content_transfer_encoding.as_deref() {
        Some("quoted-printable") => {
            let decoded =
                quoted_printable::decode(body.as_bytes(), quoted_printable::ParseMode::Robust)?;
            println!("{}", String::from_utf8_lossy(&decoded));
        }
        _ => {
            println!("{}", body);
            bail!("Content-Transfer-Encoding is not supported")
        }
    }
    Ok(())
}
