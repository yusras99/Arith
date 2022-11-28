#[derive(Clone,Debug)]
///is a struct to save the red, green, and blue pixels into floating point red, green, and blue pixels
pub struct RGBF{
  pub red: f32,
  pub green: f32,
  pub blue:f32
}
#[derive(Clone,Debug)] 
///is a struct to save the y, pb, and pr pixels 
pub struct Vid{
  pub y: f32,
  pub pb : f32,
  pub pr: f32
}
#[derive(Clone,Debug)]
///is a struct to change a, b, c ,d, pb_avg, and pr_avg 
pub struct Six_f32{
  pub a : f32,
  pub b : f32,
  pub c : f32,
  pub d : f32,
  pub pb_avg: f32,
  pub pr_avg : f32
}
 

impl Vid{
  pub fn new(y:f32,pb:f32,pr:f32) -> Self{
      Vid { y, pb, pr }
  }
}
impl RGBF{
  pub fn new(red:f32,green:f32,blue:f32) -> Self{
      RGBF {red,green,blue }
  }
}
impl Six_f32 {
   pub fn new(a : f32,b : f32,c : f32,d : f32,pb_avg: f32,pr_avg : f32)->Self{
       Six_f32{a,b,c,d,pb_avg,pr_avg}
   }
}