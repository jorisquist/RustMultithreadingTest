use std::fmt::Debug;
use hdf5::{Dataset, H5Type};
use ndarray::Array1;

pub struct StateLogger <T> where T : H5Type + Debug + Copy {
    dataset: Dataset,
    buffer: Array1<T>
}


impl<T: H5Type + Debug + Copy> StateLogger<T> {

    pub fn new(file: &hdf5::File, name: String) -> StateLogger<T> {
        let ds = file
            .new_dataset::<T>()
            .chunk((10))  // each chunk contains ny * nx elements
            .shape((0..)) // first axis is unlimited with initial size of 1
            .create(name.as_str()).unwrap();

        StateLogger::<T> {dataset: ds, buffer: Array1::default(10) }
    }

    pub fn log(&self, sample: T) {
        let size = self.dataset.shape()[0];
        self.dataset.resize((size+1));
        self.dataset.write_slice(&[sample], (size..size+1));
    }
}