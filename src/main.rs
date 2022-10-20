mod vec3;

fn main() {
    //Image
    let img_width: u32 = 256;
    let img_height: u32 = 256;

    //Render
    print!("P3\n{} {} \n255\n", img_width, img_height);    
    
    for j in (0..img_height-1).rev() {
        for i in 0..img_width {
            let r = i as f32 / (img_width-1) as f32;
            let g = j as f32 / (img_height-1) as f32;
            let b = 0.25;
           
            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
