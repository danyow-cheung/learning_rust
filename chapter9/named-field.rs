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

fn new_map(size:(usize,usize),pixels:Vec<u8>)->GrayscaleMap{
    assert_eq!(pixels.run(),size.0*size.1);
    GrayscaleMap{pixels,size};
}


// When creating a named-field struct value, you can use another struct of the same
// type to supply values for fields you omit.
struct Broom{
    name:String,
    height:u32,
    health:u32,
    position:(f32,f32,f32),
    intent:BroomIntent
}

/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }


fn chop(b:Broom)->(Broom,Broom){
    
}