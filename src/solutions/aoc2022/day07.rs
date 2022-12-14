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
    cache.get(dir).copied().unwrap_or_else(|| {
        let children = dirs.get(dir).map(|v| v.as_slice()).unwrap_or(&[]);
        let size = children
            .iter()
            .map(|c| match c {
                Child::File(f) => *f,
                Child::Dir(d) => get_size(d, dirs, cache),
            })
            .sum();

        cache.entry(dir).or_insert(size);
        size
    })
}

/// Parses the puzzle input into a tree of directories. Note that the returned [`HashMap`] uses
/// the *full path* of each directory as a key, because different directories can have
/// subdirectories with the same name.
fn parse(input: &str) -> Option<Dirs> {
    // Given the path segments making up a directory, and the name of a subdirectory, builds
    // the full path to that subdirectory as a String.
    fn subdir_path(path: &[&str], segment: &str) -> String {
        if path.is_empty() {
            segment.to_string()
        } else {
            format!("{}/{}", path.join("/"), segment)
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
                    ("dir", name) => Child::Dir(subdir_path(&path, name)),
                    (size, _) => Child::File(size.parse().ok()?),
                };

                dirs.entry(path.join("/")).or_default().push(child);
            }
        }
    }

    Some(dirs)
}
