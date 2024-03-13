//a rectangle of eight-bit grayscale pixels
struct GrayscaleMap{
    pixels: Vec<u8>,
    size:  (usize,usize)
}

let width = 1024;
let height = 576;
let image = GrayscaleMap{
    pixels:vec![0;width*height],
    size:(width,height)
}

