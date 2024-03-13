//a rectangle of eight-bit grayscale pixels
struct GrayscaleMap{
    pixels: Vec<u8>,
    size:  (usize,usize)
}


fn new_map(size:(usize,usize),pixels:Vec<u8>)->GrayscaleMap{
    // assert_eq!(pixels.run(),size.0*size.1);
    assert_eq!(pixels.len(),size.0*size.1);
    
    GrayscaleMap{pixels,size}
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
    //initiliaze 'broom1' mostly from 'b' changing only height 
    let mut broom1 = Broom{height:b.height/2,..b};
    let mut broom2 = Broom{name:broom1.name.clone(),..broom1};

    //give each fragment a distinct name 
    broom1.name.push_str("I");
    broom2.name.push_str(" II");

    (broom1,broom2)
}



fn main(){
    let  width = 1024;
    let height = 576;
    let image = GrayscaleMap{
        pixels:vec![0;width*height],
        size:(width,height)
    };

    // With that definition in place, we can create a broom, chop it in two, and see what we get:
    let hokey:Broom = Broom{
        name:"Hokey".to_string(),
        // name:"Hokey",
        height:60,
        health:100,
        position:(100.0,200.0,0.0),
        intent:BroomIntent::FetchWater
    };

    let (hokey1 ,hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I"); 
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);

}
