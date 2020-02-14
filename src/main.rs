use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::iter::Iterator;

#[derive(Clone, Copy)]

// -------------------------------level 1----------------------------- 



pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}



impl Pixel {
    /// Creates a new `Pixel`.
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r: r, g: g, b: b }
    }


    fn get_red (self) -> u8 {
        self.r
    }

    fn get_green (self) -> u8 {
        self.g
    }

    fn get_blue (self) -> u8 {
        self.b
    }

    fn invert (&self) -> Pixel{
        Pixel::new(255-&self.r, 255-&self.g ,255-&self.b)
    }

    fn grayscale (&self) -> Pixel{
        let mean = (self.r as u32 +self.g as u32+self.b as u32)/3;
        Pixel::new(mean as u8, mean as u8 ,mean as u8)
    }


}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}


impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\\\ red:{}  green:{}  blue:{} //", self.r, self.g, self.b)
    }
}



#[derive(Clone)]


// -----------------------------------level 2 --------------------------------------------------



pub struct Image {
    pub header: String ,
    pub data : Vec<Pixel>,
    pub heigth: usize,
    pub width: usize, 
    pub max_color: u8, 
}



impl Image {

        pub fn new(header:String, max_color: u8,width: usize, heigth: usize,data:Vec<Pixel>) -> Image {
                Image{header:header, max_color: max_color, data:data, heigth: heigth, width: width}
        }
    
        pub fn get_width(&self) -> usize{
            self.width
        }
          
        pub fn get_height(&self) -> usize {
            self.heigth
        }

        pub fn get_max_color(&self) -> u8 {
            self.max_color
        }

        pub fn invert(&self)->Image{
            let mut buf = vec![Pixel::new(0,0,0); self.heigth  * self.width ];
            for i in 0 ..(self.heigth  * self.width ){
            buf[i]=self.data[i].invert();
            }

            let img_inv=Image{
                data:buf,
                heigth:self.heigth,
                width:self.width,
                max_color:self.max_color,
                header:String::from("P3")

            };
            img_inv
        
        }


        pub fn gray_scale(&self)->Image{
            let mut buf = vec![Pixel::new(0,0,0); self.heigth  * self.width ];
            for i in 0 ..(self.heigth  * self.width ){
                buf[i]=self.data[i].grayscale();
                }
        
            let img_inv=Image{
                data:buf,
                heigth:self.heigth,
                width:self.width,
                max_color:self.max_color,
                header:String::from("P3")
            
            };
            img_inv
        }


        pub fn save (&self , path :&Path) -> io::Result<()>  {
            let mut pmm_file = File::create(path)?;
            let mut i= 1 ;
            let mut strr = String::new();
            strr.push_str(&self.header.to_string());
            strr.push_str("\n");
            strr.push_str(&self.width.to_string());
            strr.push_str(" ");
            strr.push_str(&self.heigth.to_string());
            strr.push_str("\n");
            strr.push_str(&self.max_color.to_string());
            strr.push_str("\n");
            while i <= self.heigth*self.width{
                    strr.push_str(&self.data[i-1].r.to_string());
                    strr.push_str(" ");
                    strr.push_str(&self.data[i-1].g.to_string());
                    strr.push_str(" ");
                    strr.push_str(&self.data[i-1].b.to_string());
                    strr.push_str("   ");
                if i % self.width == 0 {
                    strr.push_str("\n");
                }
                i = i+1
                
            }
            pmm_file.write_all(strr.as_bytes()).expect("write failed");
            Ok(())              
        }


}




// parser : to parse the file 
fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}


fn file_to_image<'a>(path: &'a str) -> Image{
    let whole_file = filename_to_string(path).unwrap();
    let liste = words_by_line(&whole_file);
    let header= liste[0][0].to_string();
    let max_color=liste[2][0].parse::<u8>().unwrap(); 
    let heigth=liste[1][1].parse::<usize>().unwrap();
    let width=liste[1][0].parse::<usize>().unwrap();
    let mut data = Vec::with_capacity(width * heigth);

    for i in 3..3+heigth{
        let mut j= 0 ;
       while j< 3*width{
        data.push(Pixel::new(liste[i][j].parse::<u8>().unwrap(),liste[i][j+1].to_string().parse::<u8>().unwrap(),liste[i][j+2].parse::<u8>().unwrap()));
        j=j+3;
        }
    }
    Image::new(header, max_color, width, heigth, data)   
}


//-------------------------------- Level 4 ----------------------------------------------------------

#[link(name = "ppma_io")]
extern "C" {
    fn square(val: i32) -> i32;
     fn ppma_write_test ( file_out_name: *const u8);
    fn ppma_read_test ( inputname: *const u8 );

}

fn readPPM (a:&str){
    unsafe{
    ppma_read_test(a.as_ptr());
}
}

fn writePPM (a:&str){
    unsafe{
    ppma_write_test(a.as_ptr());
}
}


//LEVEL 5


#[no_mangle]
pub extern fn dummy() -> usize {
    42
}



fn main() {
    writePPM("testzsssz.ppm");
    //let img = file_to_image("src/test.txt");
    //let invert = img.gray_scale();
    //let path = Path::new("src/terms212.txt");
    //img.save(&path);
    
    
} 






























// --------------------------level 3----------------------------------



































#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_red_from_pixel() {
        let pxl = Pixel::new(10,15,20);
        assert_eq!(10, pxl.get_red());
    }

    #[test]
    fn test_get_green_from_pixel() {
        let pxl = Pixel::new(10,15,20);
        assert_eq!(15, pxl.get_green());
    }

    #[test]
    fn test_get_blue_from_pixel() {
        let pxl = Pixel::new(10,15,20);
        assert_eq!(20, pxl.get_blue());
    }
    
    #[test]
    fn test_get_invert_form_pixel() {
        let pxl = Pixel::new(10,15,20);
        let pxl_invert = Pixel::new(245,240,235);
        assert!(pxl_invert==pxl.invert());
    }
    
    #[test]
    fn test_get_grayscale_form_pixel() {
        let pxl = Pixel::new(10,15,20);
        let pxl_grayscale = Pixel::new(15,15,15);
        assert!(pxl_grayscale==(pxl.grayscale()));
    }


    #[test]
    fn test_get_witdh_from_image() {
        let pxl = Pixel::new(10,15,20);
        let pxl_eq = Pixel::new(150,150,150);
        let data = vec![pxl,pxl_eq,pxl,pxl_eq,pxl,pxl_eq];
        let width = 3;
        let heigth = 2;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header, max_color, width, heigth, data);
        assert_eq!(3,image.get_width());
    }

    #[test]
    fn test_get_height_from_image() {
        let pxl = Pixel::new(10,15,20);
        let pxl_eq = Pixel::new(150,150,150);
        let data = vec![pxl,pxl_eq,pxl,pxl_eq,pxl,pxl_eq];
        let width = 3;
        let heigth = 2;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header, max_color, width, heigth, data);
        assert_eq!(2,image.get_height());
    }

    #[test]
    fn test_get_max_color_from_image() {
        let pxl = Pixel::new(10,15,20);
        let pxl_eq = Pixel::new(150,150,150);
        let data = vec![pxl,pxl_eq,pxl,pxl_eq,pxl,pxl_eq];
        let width = 3;
        let heigth = 2;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header, max_color, width, heigth, data);
        assert_eq!(255,image.get_max_color());
    }

    #[test]
    fn test_invert_from_image() {
        //creating envirenement test
        let pxl = Pixel::new(10,10,10);
        let pxl2 = Pixel::new(155,155,155);
        let data = vec![pxl,pxl2,pxl,pxl2,pxl,pxl2];

        let pxl_INV = Pixel::new(245,245,245);
        let pxl2_INV = Pixel::new(100,100,100);
        let data_INV = vec![pxl_INV,pxl2_INV,pxl_INV,pxl2_INV,pxl_INV,pxl2_INV];
        
        let width = 3;
        let heigth = 2;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header, max_color, width, heigth, data);
        let image_INV= image.invert();
        let mut test =true;

        for i in 0..6 {
            if (image_INV.data[i]==(data_INV[i])) == false {
                test = false;
            }
        }
        assert!(test);
    }

    #[test]
    fn test_file_to_image() {
        //creating envirenement test
        let pxl1 = Pixel::new(255,0,0);
        let pxl2 = Pixel::new(0,255,0);
        let pxl3 = Pixel::new(255,255,0);
        let pxl4 = Pixel::new(255,255,255);

        let data = vec![pxl1,pxl2,pxl3,pxl4];
        
        let width = 4;
        let heigth = 6;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header,max_color, width, heigth, data);

        //loading for check
        let image_from_file= file_to_image("src/test.txt");
        let mut test =true;
        

        for i in 0..4 {
            if (image_from_file.data[i]==(image.data[i])) == false {
                test = false;
            }
        }
        assert!(test);
    }

    #[test]
    fn test_save() {
        //creating envirenement test
        let pxl1 = Pixel::new(255,0,0);
        let pxl2 = Pixel::new(0,255,0);
        let pxl3 = Pixel::new(255,255,0);
        let pxl4 = Pixel::new(255,255,255);

        let data = vec![pxl1,pxl2,pxl3,pxl4];
        
        let width = 2;
        let heigth = 2;
        let header = "P3".to_string();
        let max_color = 255;
        let image = Image::new(header, max_color, width, heigth, data);

        //saving image
        let path = Path::new("src/test.txt");
        image.save(&path);

        // loading for check

        let image_for_check= file_to_image("src/test.txt");
        let mut test =true;
        

        for i in 0..4 {
            if (image.data[i]==(image_for_check.data[i])) == false {
                test = false;
            }
        }
        assert!(test);
    }

}
 