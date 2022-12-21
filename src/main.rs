mod ppm;

fn main() 
{

    const W: usize = 256;
    const H: usize = 256;

    let image      = ppm::create_image(W,H);
    let file_name  = String::from("Haloo");
    ppm::save_image(file_name,image);
}
