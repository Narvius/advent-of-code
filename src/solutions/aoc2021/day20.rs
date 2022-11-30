/// Count the number of lit pixels after 2 iterations of the enhancing algorithm.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(Image::pixels_after_iterations(input, 2))
}

/// Count the number of lit pixels after 50 iterations of the enhancing algorithm.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(Image::pixels_after_iterations(input, 50))
}

/// An image as per the puzzle description.
struct Image {
    /// The actual image data.
    data: Vec<bool>,

    /// Actual dimensions of preallocated arena.
    dimensions: (usize, usize),

    /// A second space that is written to, so as to not invalidate future reads within the same
    /// generation.
    backbuffer: Vec<bool>,

    /// The rectangle within which points are uncertain, as a pair of top left point and size. This
    /// rectangle is physically located within the center of the 2D image data, so that expanding
    /// `active_rect` equally in all directions will eventually make it match
    /// `((0, 0), dimensions)`.
    active_rect: ((usize, usize), (usize, usize)),

    /// Whether tiles outside the `active_rect` are lit up.
    outside: bool,
}

impl Image {
    /// Expands the image in the puzzle the given amount of times using the enhancing algorithm also
    /// found in the puzzle input; and returns the number of pixels that are lit at the end.
    fn pixels_after_iterations(input: &str, iterations: usize) -> usize {
        let mut image = Image::from_input(input.lines().skip(2), iterations + 1).unwrap();
        if let Some(algo) = input.lines().next() {
            for _ in 0..iterations {
                image.enhance(algo.as_bytes());
            }
        }
        image.data.into_iter().filter(|&b| b).count()
    }

    /// Applies one iteration of the enhancing algorithm to the image.
    fn enhance(&mut self, algo: &[u8]) {
        self.widen_active_rect();
        let ((x0, y0), (w, h)) = self.active_rect;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                self.add_to_backbuffer((x, y), algo);
            }
        }
        self.outside = !self.outside;
        std::mem::swap(&mut self.data, &mut self.backbuffer);
    }

    /// Expands the borders of the [`Image`] by one, taking into account whether the tiles that are
    /// now in range need to be lit up or not.
    fn widen_active_rect(&mut self) {
        self.active_rect.0 .0 -= 1;
        self.active_rect.0 .1 -= 1;
        self.active_rect.1 .0 += 2;
        self.active_rect.1 .1 += 2;

        if self.outside {
            let ((x0, y0), (w, h)) = self.active_rect;
            for x in x0..x0 + w {
                self.data[y0 * self.dimensions.1 + x] = true;
                self.data[(y0 + h - 1) * self.dimensions.1 + x] = true;
            }
            for y in y0..y0 + h {
                self.data[y * self.dimensions.1 + x0] = true;
                self.data[y * self.dimensions.1 + (x0 + w - 1)] = true;
            }
        }
    }

    /// Checks the enhancing algorithm conditions for the given coordinate, and if so, writes it
    /// to the backbuffer.
    fn add_to_backbuffer(&mut self, (x, y): (usize, usize), algo: &[u8]) {
        let ((x0, y0), (w, h)) = self.active_rect;
        let mut index = 0;
        for y in (y - 1)..=(y + 1) {
            for x in (x - 1)..=(x + 1) {
                index *= 2;
                let outside_active =
                    self.outside && (x < x0 || x >= x0 + w || y < y0 || y >= y0 + h);
                if outside_active || self.data[y * self.dimensions.1 + x] {
                    index += 1;
                }
            }
        }
        self.backbuffer[y * self.dimensions.1 + x] = algo[index] == b'#';
    }

    /// Parses an image from the relevant block in the puzzle input. `margin` describes how much
    /// empty space should be allocated around the image; this is used to avoid reallocations later
    /// in the algorithm. It should be the number of [`Image::enhance`] iterations you will call,
    /// plus one.
    fn from_input<'a>(image: impl Iterator<Item = &'a str>, margin: usize) -> Option<Image> {
        let image: Vec<_> = image.collect();
        let (w, h) = (image[0].len(), image.len());
        let dimensions = (2 * margin + w, 2 * margin + h);
        let mut data = vec![false; dimensions.0 * dimensions.1];
        let backbuffer = vec![false; dimensions.0 * dimensions.1];
        let active_rect = ((margin, margin), (w, h));

        for y in 0..h {
            for x in 0..w {
                data[(y + margin) * dimensions.1 + (x + margin)] = image[y].as_bytes()[x] == b'#';
            }
        }

        Some(Image {
            data,
            dimensions,
            backbuffer,
            active_rect,
            outside: false,
        })
    }
}
