use csc411_image::{self, RgbImage,Read,Rgb,Write};
use array2::Array2;

//Take an array2 and convert it into ppm
/// Retrieves an array and converts that array into a ppm
///
/// # Arguments:
/// * `final_arr`: An array that takes in Array2<Rgb>
pub fn write_image(final_arr:Array2<Rgb>){
  let mut vec_out = Vec::new();
  //clone rotated image
  for (_, _, v) in final_arr.iter_row_major() {
      vec_out.push(v.clone());
  }
  let outimage = RgbImage {
      pixels: vec_out,
      width: final_arr.get_width() as u32,
      height: final_arr.get_height() as u32,
      denominator: 255,
  };
  let _ = outimage.write(None).unwrap();
}
