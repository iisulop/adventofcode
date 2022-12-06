use clap::Parser;

#[derive(PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   filename: String,
}

pub fn get_filename() -> String {
    let args = Args::parse();
    args.filename
}

#[cfg(test)]
mod tests {
}
