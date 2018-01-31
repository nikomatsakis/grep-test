use extern::std::{fs::File, io::{self, BufRead, BufReader}, path::Path};

crate fn for_each_line(
    path: impl AsRef<Path>,
    mut callback: impl FnMut(&str) -> io::Result<()>,
) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line = line?;
        callback(&line)?;
    }
    Ok(())
}
