#![feature(crate_in_paths, crate_visibility_modifier, extern_absolute_paths, decl_macro,
           termination_trait, use_nested_groups, universal_impl_trait)]
#![warn(unreachable_pub)]

mod file_read;

use ::crate::file_read::for_each_line;
use ::regex::Regex;
use ::std::{
    env,
    process,
    io::{self, Write}
};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    let pattern = match args.next() {
        None => {
            // FIXME: I wanted to use `bail!` here but...
            eprintln!("usage: pattern files...");
            process::exit(1);
        }

        // FIXME: I wanted to use `?` here but...
        Some(arg) => Regex::new(&arg).expect("invalid regex"),
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for arg in args {
        for_each_line(arg, |line| {
            if pattern.is_match(line) {
                writeln!(stdout, "{}", line)?;
            }
            Ok(())
        })?;
    }

    Ok(())
}
