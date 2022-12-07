use std::collections::HashMap;

/// Find the sum of the sizes of all directories below a certain size.
pub fn one(input: &str) -> crate::Result<usize> {
    let dirs = parse(input).ok_or("parse failed")?;
    let mut cache = HashMap::new();

    Ok(dirs
        .keys()
        .map(|name| get_size(name, &dirs, &mut cache))
        .filter(|&size| size <= 100000)
        .sum())
}

/// Find the size of the smallest directory that, if deleted, would free up enough space.
pub fn two(input: &str) -> crate::Result<usize> {
    const TOTAL_SIZE: usize = 70000000;
    const REQUIRED: usize = 30000000;

    let dirs = parse(input).ok_or("parse failed")?;
    let mut cache = HashMap::new();
    let to_free = REQUIRED - (TOTAL_SIZE - get_size("", &dirs, &mut cache));

    Ok(cache
        .into_values()
        .filter(|&v| v >= to_free)
        .min()
        .unwrap_or(0))
}

/// Contains a mapping of full directory paths to children of the respective directories. Note
/// that directory paths are normalised to have no leading or trailing slashes.
type Dirs = HashMap<String, Vec<Child>>;
/// A cache mapping full directory paths to the sizes of the respective directories.
type Cache<'a> = HashMap<&'a str, usize>;

/// A directory child.
enum Child {
    /// A file child only carries its size, since the name is never used.
    File(usize),
    Dir(String),
}

/// Gets the size of a directory, simultaneously storing the size of itself and all child
/// directories in `cache`.
fn get_size<'a>(dir: &'a str, dirs: &'a Dirs, cache: &mut Cache<'a>) -> usize {
    if let Some(&size) = cache.get(dir) {
        return size;
    }

    if let Some(children) = dirs.get(dir) {
        let size = children
            .iter()
            .map(|c| match c {
                Child::File(f) => *f,
                Child::Dir(d) => get_size(d, dirs, cache),
            })
            .sum();

        cache.entry(dir).or_insert(size);
        size
    } else {
        0
    }
}

/// Parses the puzzle input into a tree of directories.
fn parse(input: &str) -> Option<Dirs> {
    // Concatenates two file paths, whilst leaving no leading or trailing slashes.
    fn concat_path(path: &[&str], name: &str) -> String {
        if path.is_empty() {
            name.to_string()
        } else {
            format!("{}/{}", path.join("/"), name)
        }
    }

    let mut dirs: Dirs = HashMap::new();
    let mut path = vec![];

    for block in input.split("$ ").skip(1) {
        if let Some(dir) = block.strip_prefix("cd ") {
            // cd command. Just changes the path we're on.
            match dir.trim() {
                "/" => path = vec![],
                ".." => {
                    path.pop();
                }
                dir => path.push(dir),
            }
        } else {
            // ls command. Lists children, which we need to store.
            for line in block.lines().skip(1) {
                let child = match line.split_once(' ')? {
                    ("dir", name) => Child::Dir(concat_path(&path, name)),
                    (size, _) => Child::File(size.parse().ok()?),
                };

                dirs.entry(path.join("/")).or_default().push(child);
            }
        }
    }

    Some(dirs)
}
