struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>,
    strides: Vec<usize>
}
impl Tensor{
    fn zeros(shape: Vec<usize>)-> Tensor{
        let total_elements:usize = shape.iter().product();
        let strides = Self::compute_strides(&shape);
        Tensor {
            data: vec![0.0;total_elements],
            shape,
            strides
        }

    }   
    fn compute_strides(shape: &[usize])-> Vec<usize>{
        let mut strides = vec![1; shape.len()];
        let mut current_stride = 1;
        for i in (0..shape.len()).rev(){
            strides[i] = current_stride;
            current_stride *= shape[i];
        }
        strides
    }   
    fn flat_index(&self, indices: &[usize])-> usize{
        assert_eq!(indices.len(), self.shape.len(), "Wrong dimensions provided");
        let mut index = 0;
        for i in 0..indices.len(){
            assert!(indices[i]< self.shape[i], "Index out of bounds at {}", i);
            index += indices[i]* self.strides[i];
        }
        index
    }
    fn get(&self, indices: &[usize]) -> f32 {
        let idx = self.flat_index(indices);
        self.data[idx]
    }
    fn set(&mut self, indices: &[usize], val: f32) {
        let idx = self.flat_index(indices);
        self.data[idx] = val;
    }
}
fn main() {

    let mut tensor_3d = Tensor::zeros(vec![2, 3, 4]);
    tensor_3d.set(&[1, 2, 3], 99.5);
    println!("Value at: {}", tensor_3d.get(&[1, 2, 3]));
    println!("Value at: {}", tensor_3d.get(&[0, 0, 0]));
    println!("Shape: {:?}", tensor_3d.shape);
    println!("Strides: {:?}", tensor_3d.strides);
}
