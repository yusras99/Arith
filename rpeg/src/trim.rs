use csc411_image::{self,Rgb,RgbImage};
use array2::Array2;

/// Takes in a image and trims the pixels, inputting the newly trimmed pixels into an array 
/// returning either the orig_arr or the even_array depending on which conditions are met
///
/// # Arguments:
/// * `orig_img`: An Rgb image that is provided from the input
pub fn create_trimmed_arr(orig_img:RgbImage)->Array2<Rgb>{
   let mut width = orig_img.width;
  let mut height = orig_img.height;
  let mut pixels = orig_img.pixels;
  //if width or height are odd, then only keep the pixels which are in the bounds of even array.
  if width % 2 != 0
  {
      let mut index = 0;
      pixels.retain(|_| {
          let px = index % width != width - 1;
          index += 1;
          px
      });
      width -=1;
  }
  if height %2 != 0 {
   let mut index = 0;
   pixels.retain(|_| {
       let px = index/width != height - 1;
       index += 1;
       px
   });
      height -=1;
     
   }
 let trimmed_arr = Array2::from_row_major(width.try_into().unwrap(),height.try_into().unwrap(),pixels).unwrap();
return trimmed_arr;
}