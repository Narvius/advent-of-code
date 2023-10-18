const WIDTH: usize = 25;
const HEIGHT: usize = 6;

/// Find the layer in the image with the least zeroes, and calculate a checksum from it.
pub fn one(input: &str) -> crate::Result<usize> {
    let counts = input.trim().as_bytes().chunks(WIDTH * HEIGHT).map(|w| {
        w.iter().fold([0, 0, 0], |mut acc, e| {
            acc[(e - b'0') as usize] += 1;
            acc
        })
    });
    counts
        .min_by_key(|v| v[0])
        .map(|[_, o, t]| o * t)
        .ok_or("no input layers".into())
}

/// Decode the image.
pub fn two(input: &str) -> crate::Result<String> {
    let image = input.trim().as_bytes().chunks(WIDTH * HEIGHT).fold(
        vec![b'2'; WIDTH * HEIGHT],
        |mut top, bottom| {
            for (t, b) in top.iter_mut().zip(bottom) {
                if *t == b'2' {
                    *t = *b;
                }
            }
            top
        },
    );

    Ok(crate::common::pixel_display(WIDTH, HEIGHT, |x, y| {
        image[y * WIDTH + x] == b'1'
    }))
}
