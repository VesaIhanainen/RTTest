use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Color
{
    r: i32,
    g: i32,
    b: i32,
}
pub struct Image
{
    width  : usize,
    heigth : usize,
    data   : Box<[Color]>,
}
pub fn create_image(w: usize, h: usize) -> Image
{
    let data = vec![Color{r:0,g:0,b:0}; h*w];
    let _image = Image {
        width   : w,
        heigth  : h,
        data    : data.into_boxed_slice()     
    };
    _image
}
pub fn save_image(image_name: String, image: Image) -> std::io::Result<()>
{
    let mut file = File::create(image_name+".ppm")?;
    let ppm_header = format!("P3\n{} {}\n{}\n",image.width, image.heigth, 255);
    file.write_all(ppm_header.as_bytes())?;
    Ok(())
    
}

