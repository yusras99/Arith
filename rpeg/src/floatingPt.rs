use csc411_image::{self,Rgb};
use array2::Array2;
use crate::{types:: RGBF};
/// Retrieves an array of pixels and converts the image given from a Rgb to a RGBF and
/// returns the new array containing those newely transformed pixels.
///
/// # Arguments:
/// * `trimmed_arr`: An array contaning Array2<rgb> and the pixels within
/// * 'denom': A floating point denominator
pub fn create_rgbf_arr( trimmed_arr:Array2<Rgb>, denom: f32)->Array2<RGBF>{
  let height = trimmed_arr.get_height();
  let width = trimmed_arr.get_width();
  let mut rgbf_vec:Vec<RGBF> = Vec::new();
   //Run an iterator throughout the trimed array and change each pixel to a floating point.
  //Create a tuple of this and store this in the trimmed arr.
  for (c, r, v) in trimmed_arr.iter_row_major() {
      let d = v.clone();
      let (r, g, b) = match d {
          Rgb { red, green, blue } => (red, green, blue),
      };
      let float_pixel:RGBF = RGBF{red:(r as f32/denom),green:(g as f32/denom),blue:(b as f32/denom)};
      rgbf_vec.push(float_pixel);
    
  }
  let rgbF_arr = Array2::from_row_major(width,height,rgbf_vec).unwrap();
  return rgbF_arr;
}
/// Retrieves an array of pixels and converts the image given from a RGBF to a Rgb and
/// returns the new array containing those newely transformed pixels.
///
/// # Arguments:
/// * `rgbf_arr`: An array contaning Array2<RGBF> and the pixels within
pub fn create_rgb_arr(rgbf_arr:Array2<RGBF>)->Array2<Rgb>{
  let height = rgbf_arr.get_height();
  let width = rgbf_arr.get_width();
  let  val = Rgb{red:0, green:0,blue:0};
  let mut rgb_vec = Vec::new();
  for (c, r, v) in rgbf_arr.iter_row_major() {
      //vec_rgb.push(v.clone());
      let d = v.clone();
      //*d = v.clone();
      let (r, g, b) = match d {
          RGBF { red, green, blue } => (red, green, blue),
      };
      let rgb_pix:Rgb = Rgb{red:(r*255.0).clamp(0.0, 255.0).floor() as u16,green:(g*255.0).clamp(0.0, 255.0).floor() as u16,blue:(b*255.0).clamp(0.0, 255.0).floor() as u16};
      rgb_vec.push(rgb_pix);
  }
  let rgb_arr = Array2::from_row_major(width,height,rgb_vec).unwrap();
  return rgb_arr;
}
