use array2::Array2;
use crate::types::{RGBF,Vid};

fn rgbf_to_vid_math(rgbf_pix:RGBF)->Vid{
  let vid_pix = Vid::new(0.299 * rgbf_pix.red + 0.587 * rgbf_pix.green + 0.114 * rgbf_pix.blue,
     -0.168736 * rgbf_pix.red - 0.331264 * rgbf_pix.green + 0.5 * rgbf_pix.blue, 
     0.5 * rgbf_pix.red - 0.418688 * rgbf_pix.green - 0.081312 * rgbf_pix.blue);
  return vid_pix;
}
 
pub fn rgbf_to_vid(mut rgbf_arr:Array2<RGBF>)->Array2<Vid>{
  let height = rgbf_arr.get_height();
  let width = rgbf_arr.get_width();
  let mut vid_vec:Vec<Vid> = Vec::new();
  for (c, r, v) in rgbf_arr.iter_mut_row_major() {
      let d = v.clone();
      let (r, g, b) = match d {
          RGBF { red,green,blue } => (red,green,blue),
      };
      let vid_pixel = rgbf_to_vid_math(d);
      vid_vec.push(vid_pixel);
  }
  let vid_arr = Array2::from_row_major(width,height,vid_vec).unwrap();
  return vid_arr;
}


fn vid_to_rgbf_math(vid_pixel:Vid)->RGBF{
    let rgbf_pix = RGBF::new(1.0 * vid_pixel.y + 0.0 *vid_pixel.pb + 1.402 * vid_pixel.pr, 
     1.0 * vid_pixel.y - 0.344136 * vid_pixel.pb - 0.714136 * vid_pixel.pr,
     1.0 * vid_pixel.y + 1.772 * vid_pixel.pb +0.0 * vid_pixel.pr); 
   
    return rgbf_pix;
  
}

pub fn vid_to_Rgbf(vid_arr:Array2<Vid>)->Array2<RGBF>{
  let height = vid_arr.get_height();
  let width = vid_arr.get_width();
  let mut rgbf_vec:Vec<RGBF> = Vec::new();
  for (c, r, v) in vid_arr.iter_row_major() {
      let d = v.clone();
      let (y, pb, pr) = match d {
       Vid{ y,pb,pr } => (y,pb,pr),
      };
      let rgbf_pixel = vid_to_rgbf_math(d);
      rgbf_vec.push(rgbf_pixel);
  }
  let rgbf_arr = Array2::from_row_major(width,height,rgbf_vec).unwrap();
  return rgbf_arr;
}
