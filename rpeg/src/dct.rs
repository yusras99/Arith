//use alloc::collections;
use array2::Array2;
use csc411_arith;
use crate::types::{Vid, Six_f32};
use std::{ops::Shr, vec};
use bitpack::bitpack;
use std::io::{Write};
use std::fs;
use std::convert::TryInto;
use scan_fmt::scan_fmt;
//Take the vector of all pixels having a,b,c,d, pb avg and pr avg values and convert them into their chroma indices
//in order to calculate that, multiply each value with the highest bit it can represent. 50 in calculating b,c,d index comes from dividing 15 by 0.3.
//Then pack them into byte words.
fn encode(vec: &Vec<Six_f32>, width: usize, height: usize){
println!("Compressed image format 2\n{} {}", width as u32, height as u32);
for pix in vec{
 let a_ndx = (pix.a.clamp(0.0, 1.0) * ((1<<9)-1) as f32).floor() as u32;
 let b_ndx = (pix.b.clamp(-0.3, 0.3) * 50.0 as f32).floor() as u32;
 let c_ndx = (pix.c.clamp(-0.3, 0.3) * 50.0 as f32).floor() as u32;
 let d_ndx = (pix.d.clamp(-0.3, 0.3) * 50.0 as f32).floor() as u32;
 let pb_avg = csc411_arith::index_of_chroma(pix.pb_avg);
 let pr_avg = csc411_arith::index_of_chroma(pix.pr_avg);
 let mut packed = 0_u32;
 packed = bitpack::newu(packed as u64, 9, 23, a_ndx as u64).unwrap() as u32;
 packed = bitpack::news(packed as u64, 5, 18, b_ndx as i64).unwrap() as u32;
 packed = bitpack::news(packed as u64, 5, 13, c_ndx as i64).unwrap() as u32;
 packed = bitpack::news(packed as u64, 5, 8, d_ndx as i64).unwrap() as u32;
 packed = bitpack::newu(packed as u64, 4, 4, pb_avg as u64).unwrap() as u32;
 packed = bitpack::newu(packed as u64, 4, 0, pr_avg as u64).unwrap() as u32;
 std::io::stdout().write_all(&packed.to_be_bytes()).unwrap();
}
}
 
/// Retrieves an array of pixels and iterates through that array grabbing the pixels
/// and changing the values to their chroma index
///
/// # Arguments:
/// * `vid_array`: An array containing Array2<Vid>
pub fn quant_arr(vid_array:Array2<Vid>){
  //Iterate through the array which contains component video values using step by so you can grab the pixels of a 2 by 2 block.
  //use get_element function to index into the array and grab the pixels.
  //Average the pb and pr and apply DCT on y values to get a,b,c,d and encode them.
  let mut twoByTwo_vec = Vec::new();
  for h in (0..vid_array.get_height()).step_by(2) {
     for w in (0..vid_array.get_width()).step_by(2) {
      let pix1 = vid_array.get_element(w,h).unwrap();
      let pix2 = vid_array.get_element(w+1,h).unwrap();
      let pix3 = vid_array.get_element(w,h+1).unwrap();
      let pix4 = vid_array.get_element(w+1,h+1).unwrap();
      let avgPb = (pix1.pb + pix2.pb + pix3.pb + pix4.pb)/4.0;
      let avgPr = (pix1.pr + pix2.pb + pix3.pb + pix4.pb)/4.0;
      let a = (pix4.y + pix3.y + pix2.y + pix1.y)/4.0;
      let b = (pix4.y + pix3.y - pix2.y - pix1.y)/4.0;
      let c = (pix4.y - pix3.y + pix2.y - pix1.y)/4.0;
      let d = (pix4.y - pix3.y - pix2.y + pix1.y)/4.0;
      let twoByTwo_pix = Six_f32::new(a,b,c,d,avgPb,avgPr);
      //let chroma = quantization(2by2_pix);
      twoByTwo_vec.push(twoByTwo_pix);
      }
  }
  encode(&twoByTwo_vec, vid_array.get_width(), vid_array.get_height())
}

//The decompression functions

//this function just takes 32 byte words and extract a,b,c,d,pb, and pr from it.
fn unpack(byte_dec:u32)->Six_f32{
//1<<9 gives us 512, and subtract 1 to get 511
let a = ((bitpack::getu(byte_dec as u64, 9, 23)))as f32 / ((1<<9)-1) as f32;
let b = ((bitpack::gets(byte_dec as u64, 5, 18))as f32)/50.0;
let c = ((bitpack::gets(byte_dec as u64, 5, 13)))as f32/50.0;
let d = ((bitpack::gets(byte_dec as u64, 5, 8)))as f32 /50.0;
let pb_avg = csc411_arith::chroma_of_index(bitpack::getu(byte_dec as u64, 4, 4) as usize) ;
let pr_avg = csc411_arith::chroma_of_index(bitpack::getu(byte_dec as u64, 4, 0) as usize);
let decoded = Six_f32::new(a,b,c,d,pb_avg,pr_avg);
return decoded;
}
//This function will unpack the word and then convert the chroma indices back to chroma vals
//push these values into an array2 of the six values a,b,c,d,pb,pr
/// Retrieves an a file from the command line and unpacks the word, converting it into the chromas indices
/// and pushing these values into array2
///
/// # Arguments:
/// * `filename`: Takes a filename that it is a type of reference to a string that is given on the
/// command line
pub fn decode(filename:&str)->Option<Array2<Vid>>{
//read from a certain location
let f = fs::read(filename);
let matchf = match f {
    Ok(x)=>{
      let utf_str = match std::str::from_utf8(&x[..]){
        Ok(str)=>str,
        Err(error)=> std::str::from_utf8(&x[..error.valid_up_to()]).unwrap()
    };
    let (width,height) = scan_fmt!(utf_str,"Compressed image format 2\n{} {}\n",u32,u32).unwrap();
    let start = format!("Compressed image format 2\n{} {}\n",width,height).len();
    let mut vec_chroma = Vec::new();
    //start reading the byte after reading the utf chars
    for byte_slice in x[start..].chunks(4){
      let byte_dec = u32::from_be_bytes(byte_slice.try_into().unwrap());
      let pix_six = unpack(byte_dec);
      vec_chroma.push(pix_six);
    }
    let vid_arr = non_quant_arr(vec_chroma, width, height);
    return Some(vid_arr);
  },
  Err(error)=>println!("{:?}",error),
 };
return None;
}
//Read the array with chroma values and convert the a,b,c,d vals in ys and make vid pixels
//store the vid pixel in a new array and return that
fn non_quant_arr(chroma_vec:Vec<Six_f32>,width:u32,height:u32)->Array2<Vid>{
  //let mut twoBytwo_vec = Vec::new();
  let mut vec_final = vec![Vid::new(0.0, 0.0, 0.0); width as usize*height as usize];
  //let mut count = 0;
  for (ndx, v) in chroma_vec.into_iter().enumerate(){
      let y1 = v.a - v.b - v.c + v.d;
      let y2 = v.a - v.b + v.c - v.d;
      let y3 = v.a + v.b - v.c - v.d;
      let y4 = v.a + v.b + v.c + v.d;
      let vid_pix1 = Vid::new(y1,v.pb_avg,v.pr_avg);
      let vid_pix2 = Vid::new(y2,v.pb_avg,v.pr_avg);
      let vid_pix3 = Vid::new(y3,v.pb_avg,v.pr_avg);
      let vid_pix4 = Vid::new(y4,v.pb_avg,v.pr_avg);
      println!("{}", ndx);
      let col = 2 * (ndx % (width/2) as usize);
      let row = 2 * (ndx / (width/2) as usize);
      //let col = 2 * (ndx % (width/2) as usize);
      //println!("{}",ndx);
      //let row = (ndx-1 / (width) as usize);

      vec_final[row*width as usize + col ] = vid_pix1;
      vec_final[row*width as usize + col + 1] = vid_pix2;
      vec_final[(row + 1)*width as usize + col] =vid_pix3;
      vec_final[(row + 1)*width as usize + col + 1] =vid_pix4;
  }
  let non_quant_arr = Array2::from_row_major(width as usize,height as usize,vec_final).unwrap();
  return non_quant_arr;
}