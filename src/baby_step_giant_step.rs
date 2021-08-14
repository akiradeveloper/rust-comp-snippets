use cargo_snippet::snippet;

use std::collections::HashMap;

#[snippet(BabyStepGiantStep)]
pub trait BSGSable {
	type T: std::fmt::Debug;
	type K: std::hash::Hash + std::cmp::Eq;
	fn inv(x: &Self::T, mo: u64) -> Self::T;
	fn unit() -> Self::T;
	fn multiply(x: &Self::T, y: &Self::T, mo: u64) -> Self::T;
	fn unique_key_for(x: &Self::T) -> Self::K;
}

/// a^x = b (mod m)
/// を解く。
/// 計算量: O(root M)

#[snippet(BabyStepGiantStep)]
pub fn solve_bsgs<M: BSGSable>(a: M::T, b: M::T, mo: u64) -> Option<u64> {
	let mut r = 1;
	while r*r < mo {
		r += 1;
	}
	// a^j
	let mut baby_step = vec![];
	baby_step.push(M::unit());
	for j in 1..r {
		let prev = &baby_step[j as usize-1];
		let next = M::multiply(prev, &a, mo);
		baby_step.push(next);
	}
	let mut baby_step_k2j = HashMap::new();
	for j in 0..r {
		let x = &baby_step[j as usize];
		let k = M::unique_key_for(x);
		baby_step_k2j.insert(k, j);
	}

	// (a^-r)^i
	let mut giant_step = vec![];
	// a^-1
	let a_inv = M::inv(&a, mo);
	// a^-r
	let mut a_inv_pow_r = M::unit();
	for _ in 0..r {
		a_inv_pow_r = M::multiply(&a_inv_pow_r, &a_inv, mo);
	}
	giant_step.push(M::unit());
	for i in 1..r {
		let prev = &giant_step[i as usize-1];
		let next = M::multiply(&prev, &a_inv_pow_r, mo);
		giant_step.push(next);
	}

	for i in 0..r {
		let gs = &giant_step[i as usize];
		let tgt = M::multiply(&b, &gs, mo);
		let key = M::unique_key_for(&tgt);
		if let Some(j) = baby_step_k2j.get(&key) {
			return Some(i*r + j);
		}
	}
	return None;
}

#[test]
fn test_bsgs() {
	struct M;
	impl BSGSable for M {
		type T = u64;
		type K = u64;

		fn unit() -> u64 {
			1
		}
		fn multiply(x: &u64, y: &u64, mo: u64) -> u64 {
			*x * *y % mo
		}
		fn inv(x: &u64, mo: u64) -> u64 {
			crate::number::modinv(*x as i64, mo as i64) as u64
		}
		fn unique_key_for(x: &u64) -> u64 {
			*x
		}
	}
	let bx = [
		(608377687, 111),
		(245166051, 1111),
		(416655179, 11111),
		(235632030, 111111),
		(109865711, 1111111),
		(854851041, 11111111),
	];
	for (b,x) in bx {
		let ans = solve_bsgs::<M>(2, b, 1_000_000_007);
		assert_eq!(ans, Some(x));
	}
}