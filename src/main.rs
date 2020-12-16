use std::fs::{read_to_string, File};
use structopt::StructOpt;
use std::io::Write;

mod sf_objects;
use sf_objects::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sf-simplify", about = "Removes extra thick crud from SF object describes")]
struct Opts {
  /// Input file path
  #[structopt(short, long)]
  input: String,

  /// Output file path
  #[structopt(short, long)]
  output: String,
}

fn main() -> anyhow::Result<()> {
  let args = Opts::from_args();

  println!("Reading input file...");
  let data = read_to_string(args.input)?;
  let parsed: SFObject = serde_json::from_str(&data)?;

  println!("Writing output file...");
  let mut output = File::create(args.output)?;
  output.write_all(&serde_json::to_string_pretty(&parsed)?.as_bytes())?;

  Ok(())
}
