#[derive(Debug,  Clone, Eq, PartialEq)]
pub struct Array2<T>{
    width: usize,
    height: usize,
    pub array: Vec<T>
}
impl<T: Clone> Array2<T> {
    //a constructor that allows us to initialize array with one value. 
    pub fn new(width:usize,height:usize,val:T ) -> Self {
        let data = vec![val; width * height];
        Array2{
                width,
                height,
                array : data,
            }
    }
    pub fn from_row_major(width:usize,height:usize,vec: Vec<T>) -> Option<Array2<T>>{
        /* A constructor that populates the data structure */
        if width*height!= vec.len(){
            None
        }else{
            Some(Array2{
                width,
                height,
                array : vec,
            })
        
        }
    }
    pub fn compute_index(&self,col:usize,row:usize) -> Option<usize>{
        if row >= self.height || col >= self.width{
            None
        }
        else{
            Some(row* self.width + col)
        }
    }
    
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        //row, column, value
        self.array
            .iter()
            .enumerate()
            .map(move |(i, v)| (i % self.width, i / self.width, v))
    }

    pub fn iter_mut_row_major(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        //row, column, value
        let width  = self.width;
        self.array
            .iter_mut()
            .enumerate()
            .map(move |(i, v)| (i % width, i / width, v))
    }

    pub fn iter_col_major(&self)-> impl Iterator<Item = (usize,usize,&T)>{
        (0..self.width)
            //c is the column index, this helps us to get the index of all the first elements of each columns
            .map(move |c| (c, self.array.iter().skip(c)))
            //it flattens all the columns
            .flat_map(move|(c,col)|{
                //and then we get each element of each column
                col.step_by(self.width)
                    .enumerate()
                    .map(move|(r,val)|(c,r,val))
            })
    }

    pub fn get_element(&self,col:usize,row:usize) -> Option<&T>{
        let new_index = self.compute_index(col, row)?;
        let v = self.array.get(new_index)?;
        /* Take the vector,compute the index, use that index to search for the element in the vectorcall the iterator method on it 
        call the iterator method on the 
        */
        return Some(v);
       }
       
    pub fn get_mut(&mut self, col:usize,row:usize) -> Option<&mut T>{
        let new_index = self.compute_index(col, row)?;
        let v = self.array.get_mut(new_index)?;
        /* Take the vector,compute the index, use that index to search for the element in the vectorcall the iterator method on it 
        call the iterator method on the 
        */
        return Some(v);
    
    } 
    pub fn get_height(&self) -> usize{
        self.height
    }
    pub fn get_width(&self) -> usize{
        self.width
    }
}


#[cfg(test)]
mod tests {
    use crate::Array2;
   #[test]
    fn create_and_access() {
        let a = Array2::from_row_major(3,2,vec![1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(*a.get_element(2, 0).unwrap(), 3);
    }

    #[test]
    fn modify_and_access() {
        let mut a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let i = a.get_mut(2, 0).unwrap();
        *i = 99;
        assert_eq!(*a.get_element(2, 0).unwrap(), 99);
    }

    #[test]
    fn access_out_of_bounds() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get_element(3, 0);
        assert_eq!(v, None);
    }
    #[test]
    fn access_in_bounds() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get_element(2, 0);
        assert_eq!(v, Some(&3));
    }

    #[test]
    fn modify_out_of_bounds() {
        let mut a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v = a.get_mut(3, 0);
        assert_eq!(v, None);
    }

    #[test]
    fn sum_via_fold() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let v: i32 = a.iter_row_major().fold(0_i32, |acc, (_, _, x)| acc + x);
        assert_eq!(v, 21);
    }

    #[test]
    fn iter_row_major() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let mut b = a.iter_row_major();

        assert_eq!(b.next(), Some((0, 0, &1)));
        assert_eq!(b.next(), Some((1, 0, &2)));
        assert_eq!(b.next(), Some((2, 0, &3)));

        assert_eq!(b.next(), Some((0, 1, &4)));
        assert_eq!(b.next(), Some((1, 1, &5)));
        assert_eq!(b.next(), Some((2, 1, &6)));

        assert_eq!(b.next(), None);
    }

    #[test]
    fn iter_col_major() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let mut b = a.iter_col_major();

        assert_eq!(b.next(), Some((0, 0, &1)));
        assert_eq!(b.next(), Some((0, 1, &4)));

        assert_eq!(b.next(), Some((1, 0, &2)));
        assert_eq!(b.next(), Some((1, 1, &5)));

        assert_eq!(b.next(), Some((2, 0, &3)));
        assert_eq!(b.next(), Some((2, 1, &6)));

        assert_eq!(b.next(), None);
    }
    /*#[test]
    fn trim() {
        let a = Array2::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();

        let b = a.trim_last_row().unwrap();
        let b_correct = Array2::from_row_major(3, 1, vec![1, 2, 3]).unwrap();
        assert_eq!(b, b_correct);

        let c = a.trim_last_col().unwrap();
        let c_correct = Array2::from_row_major(2, 2, vec![1, 2, 4, 5]).unwrap();
        assert_eq!(c, c_correct);
    }

    #[test]
    fn blocks() {
        let a = Array2::from_row_major(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let mut b = a.get_block(0, 0, 2, 2);

        assert_eq!(b.next(), Some(&1));
        assert_eq!(b.next(), Some(&2));
        assert_eq!(b.next(), Some(&4));
        assert_eq!(b.next(), Some(&5));
        assert_eq!(b.next(), None);
    }

    #[test]
    fn iter_blocks() {
        let a = Array2::from_row_major(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let mut b = a.iter_square_blocks(2);

        let mut b0 = b.next().unwrap();
        let mut b1 = b.next().unwrap();
        let mut b2 = b.next().unwrap();
        let mut b3 = b.next().unwrap();
        assert!(b.next().is_none());

        assert_eq!(b0.next(), Some(&1));
        assert_eq!(b0.next(), Some(&2));
        assert_eq!(b0.next(), Some(&4));
        assert_eq!(b0.next(), Some(&5));
        assert_eq!(b0.next(), None);

        assert_eq!(b1.next(), Some(&3));
        assert_eq!(b1.next(), Some(&6));
        assert_eq!(b1.next(), None);

        assert_eq!(b2.next(), Some(&7));
        assert_eq!(b2.next(), Some(&8));
        assert_eq!(b2.next(), None);

        assert_eq!(b3.next(), Some(&9));
        assert_eq!(b3.next(), None);
    }*/
}