use csc411_image::{self, RgbImage,Read,Rgb};
use array2::Array2;

use crate::trim::create_trimmed_arr;
use crate::writeImg::{write_image};
use crate::floatingPt::{create_rgb_arr,create_rgbf_arr};
use crate::compVid::{rgbf_to_vid, vid_to_Rgbf};
use crate::dct::{decode, quant_arr};

pub fn compress(filename: &str) {
    //Store the image in an array
    let orig_img = RgbImage::read(Some(filename)).unwrap();
    let denominator: f32 = orig_img.denominator as f32;
    //trim the array if needed
    let trimmed_arr = create_trimmed_arr(orig_img);
    let float_arr = create_rgbf_arr(trimmed_arr,denominator);
    let comp_vid_arr = rgbf_to_vid(float_arr);
    quant_arr(comp_vid_arr);
}
pub fn decompress(filename: &str) { 
    let vid_arr = decode(filename).unwrap();
    let rgbf_arr = vid_to_Rgbf(vid_arr);
    let rgb_arr = create_rgb_arr(rgbf_arr);
    write_image(rgb_arr);

}
//./rpeg -c moss_small.ppm >> output.txt