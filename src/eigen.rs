use std::ops;
use std::fmt;


pub struct Vector {
    data_: Vec<f64>,
    dim_: usize,
}

impl Vector {
    pub fn from(data: Vec<f64>) -> Vector {
        Vector {
            dim_: data.len(),
            data_: Vec::from(data),
        }
    }

    pub fn new(dim: usize) -> Vector {
        Vector {
            dim_: dim,
            data_: vec![0.0; dim],
        }
    }

    pub fn add(&mut self, other: &Vector) -> Result<Vector, String>{
        if other.dim() != self.dim() {
            return Err("Vector dimensions do not match".to_string())
        }
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] + other.data_[i]);
        }
        Ok(Vector::from(result))
    }

    pub fn sub(&mut self, other: &Vector) -> Result<Vector, String>{
        if other.dim() != self.dim() {
            return Err("Vector dimensions do not match".to_string())
        }
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] - other.data_[i]);
        }
        Ok(Vector::from(result))
    }

    pub fn mul(&mut self, other: &Vector) -> Result<f64, String>{
        if other.dim() != self.dim() {
            return Err("Vector dimensions do not match".to_string())
        }
        let mut result = 0.0;
        for i in 0..self.dim_ {
            result += self.data_[i] * other.data_[i];
        }
        Ok(result)
    }

    pub fn scale(&mut self, other: f64) -> Result<Vector, String>{
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] * other);
        }
        Ok(Vector::from(result))
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("[");
        for i in 0..self.dim_ {
            result.push_str(&self.data_[i].to_string());
            if i < self.dim_ - 1 {
                result.push_str(" ");
            }
        }
        result.push_str("]");
        result
    }

    pub fn dim(&self) -> usize {
        self.dim_
    }

    pub fn raw_data(&self) -> &Vec<f64> {
        &self.data_
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] * other);
        }
        Vector::from(result)
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        if other.dim() != self.dim() {
            panic!("Vector dimensions do not match");
        }
        let mut result = 0.0;
        for i in 0..self.dim_ {
            result += self.data_[i] * other.data_[i];
        }
        result
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        if other.dim() != self.dim() {
            panic!("Vector dimensions do not match");
        }
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] + other.data_[i]);
        }
        Vector::from(result)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        if other.dim() != self.dim() {
            panic!("Vector dimensions do not match");
        }
        let mut result = Vec::new();
        for i in 0..self.dim_ {
            result.push(self.data_[i] - other.data_[i]);
        }
        Vector::from(result)
    }
}

impl ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        if index >= self.dim_ {
            panic!("Index out of bounds");
        }
        &self.data_[index]
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        if index >= self.dim_ {
            panic!("Index out of bounds");
        }
        &mut self.data_[index]
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/*
pub struct Matrix {
    data_: Vec<Vector>,
    nrows_: usize,
    ncols_: usize,
}
*/