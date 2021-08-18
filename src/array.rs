use cargo_snippet::snippet;

#[snippet("Array")]
#[derive(Debug, Clone, PartialEq)]
pub struct Array1d<T> {
    v: Vec<T>,
}
#[snippet("Array")]
impl <T> Array1d<T> {
	pub fn new(v: Vec<T>) -> Self {
		Array1d { v }
	}
}
#[snippet("Array")]
impl <T> std::ops::Index<i64> for Array1d<T> {
	type Output = T;
	fn index(&self, i: i64) -> &Self::Output {
		&self.v[i as usize]
	}
}
#[snippet("Array")]
impl <T> std::ops::IndexMut<i64> for Array1d<T> {
	fn index_mut(&mut self, i: i64) -> &mut Self::Output {
		&mut self.v[i as usize]
	}
}
#[snippet("Array")]
#[derive(Debug, Clone, PartialEq)]
pub struct Array2d<T> {
	v: Vec<Array1d<T>>,
}
#[snippet("Array")]
impl <T> Array2d<T> {
	pub fn new(v: Vec<Vec<T>>) -> Self {
		let mut vv = vec![];
		for row in v {
			vv.push(Array1d::new(row));
		}
		Array2d { v: vv }
	}
}
#[snippet("Array")]
impl <T> std::ops::Index<i64> for Array2d<T> {
	type Output = Array1d<T>;
	fn index(&self, i: i64) -> &Self::Output {
		&self.v[i as usize]
	}
}
#[snippet("Array")]
impl <T> std::ops::IndexMut<i64> for Array2d<T> {
	fn index_mut(&mut self, i: i64) -> &mut Self::Output {
		&mut self.v[i as usize]
	}
}

#[test]
fn test_array() {
	let mut a = Array2d::new(vec![
		vec![0,0,0],
		vec![0,0,0],
	]);
	for i in 0..2i64 {
		for j in 0..3i64 {
			a[i][j] = 1+i+j;
		}
	}
	dbg!(&a);
}