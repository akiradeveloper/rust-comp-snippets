/// https://github.com/hatoo/competitive-rust-snippets

use std;

#[snippet = "Vector2D"]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Vector2D(f64, f64);

#[snippet = "Vector2D"]
impl Vector2D {
    pub fn add(a: f64, b: f64) -> f64 {
        let c = a + b;
        if c.abs() < 1e-10 {
            0.0
        } else {
            c
        }
    }
    pub fn dot(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.0, self.1 * other.1)
    }
    pub fn det(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.1, -self.1 * other.0)
    }
    pub fn len(&self) -> f64 {
        f64::sqrt((self.0).powi(2) + (self.1).powi(2))
    }
    pub fn unit(self) -> Vector2D {
        let l = self.len();
        Vector2D(self.0 / l, self.1 / l)
    }
    #[doc = "orthogonal vector"]
    pub fn normal(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }
    #[doc = "bisection of the angle"]
    pub fn bisect(a: Vector2D, b: Vector2D) -> Vector2D {
        (a.unit() + b.unit()).unit()
    }
}

#[snippet = "Vector2D"]
impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, rhs.0), Vector2D::add(self.1, rhs.1))
    }
}

#[snippet = "Vector2D"]
impl std::ops::Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, -rhs.0), Vector2D::add(self.1, -rhs.1))
    }
}

#[snippet = "Vector2D"]
impl std::ops::Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(rhs * self.0, rhs * self.1)
    }
}

#[snippet = "Vector2D"]
impl std::ops::Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}

#[snippet = "Circle"]
pub struct Circle {
    center: Vector2D,
    radius: f64,
}

#[snippet = "Circle"]
impl Circle {
    pub fn inner_circle(a: Vector2D, b: Vector2D, c: Vector2D) -> Circle {
        let a_bisect = Line2D {
            p: a,
            d: Vector2D::bisect(a-b, a-c),
        };
        let b_bisect = Line2D {
            p: b,
            d: Vector2D::bisect(b-a, b-c),
        };

        let center = Line2D::intersection(a_bisect, b_bisect);
        let ab = Line2D {
            p: a,
            d: b-a,
        };
        let radius = ab.distance(center);
        Circle {
            center: center,
            radius: radius,
        }
    }
    pub fn outer_circle(a: Vector2D, b: Vector2D, c: Vector2D) -> Circle {
        let ubn = Line2D {
             p: (a+b) / 2.,
             d: (a-b).normal()
        };
        let vbn = Line2D {
            p: (b+c) / 2.,
            d: (b-c).normal(),
        };

        let center = Line2D::intersection(ubn, vbn);
        let radius = (a - center).len();

        Circle {
            center: center,
            radius: radius,
        }
    }
    pub fn outer_circle_stable(x: Vector2D, y: Vector2D, z: Vector2D) -> Option<Circle> {
        let a = (y-z).len();
        let a2 = a*a;
        let b = (x-z).len();
        let b2 = b*b;
        let c = (x-y).len();
        let c2 = c*c;

        let eps = 1e-9;
        if a+b-c < eps { return None }
        if b+c-a < eps { return None } 
        if c+a-b < eps { return None }

        let X = x*(a2*(b2+c2-a2)) + y*(b2*(c2+a2-b2)) + z*(c2*(a2+b2-c2));
        let Y = a2*(b2+c2-a2) + b2*(c2+a2-b2) + c2*(a2+b2-c2);
        let center = X / Y;
        let radius = (x-center).len();
        Some(Circle {
            center: center,
            radius: radius
        })
    }
}

use crate::total::Total;

#[snippet = "convex_hull"]
#[allow(dead_code)]
fn convex_hull(vs: &[Vector2D]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..vs.len()).collect();
    idx.sort_by_key(|&i| Total((vs[i].0, vs[i].1)));

    let mut res = Vec::new();

    for &i in &idx {
        while res.len() > 1
            && Vector2D::det(
                vs[res[res.len() - 1]] - vs[res[res.len() - 2]],
                vs[i] - vs[res[res.len() - 1]],
            ) <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }
    let t = res.len();

    for &i in idx.iter().rev().skip(1) {
        while res.len() > t
            && (vs[res[res.len() - 1]] - vs[res[res.len() - 2]]).det(vs[i] - vs[res[res.len() - 1]])
                <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }

    res.pop();
    res
}

#[snippet = "closest_pair"]
pub fn closest_pair(ps: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    fn d(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
    }

    fn rec(x_sort: &[(f64, f64)], y_sort: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
        if x_sort.len() <= 3 {
            let mut min_d = std::f64::MAX;
            let mut pair = ((0.0, 0.0), (0.0, 0.0));
            for (i, &p1) in x_sort.iter().enumerate() {
                for (j, &p2) in x_sort.iter().enumerate() {
                    if i != j {
                        let dist = d(p1, p2);
                        if dist < min_d {
                            min_d = dist;
                            pair = (p1, p2);
                        }
                    }
                }
            }

            return pair;
        }

        let mid = x_sort.len() / 2;
        let pivot = x_sort[mid].0;

        let q_x = &x_sort[..mid];
        let r_x = &x_sort[mid..];

        let mut q_y = Vec::with_capacity(mid);
        let mut r_y = Vec::with_capacity(x_sort.len() - mid);

        for &(x, y) in y_sort {
            if x < pivot {
                q_y.push((x, y));
            } else {
                r_y.push((x, y));
            }
        }

        let pair1 = rec(q_x, &q_y);
        let pair2 = rec(r_x, &r_y);

        let w = d(pair1.0, pair1.1).min(d(pair2.0, pair2.1));
        let s: Vec<(f64, f64)> = y_sort
            .iter()
            .filter(|&&(x, _)| (pivot - x).abs() <= w)
            .cloned()
            .collect();

        let mut min_d = w;
        let mut pair = if d(pair1.0, pair1.1) < d(pair2.0, pair2.1) {
            pair1
        } else {
            pair2
        };

        for (i, &p1) in s.iter().enumerate() {
            for &p2 in s[i + 1..].iter().take(15) {
                let dist = d(p1, p2);
                if dist < min_d {
                    min_d = dist;
                    pair = (p1, p2);
                }
            }
        }
        pair
    }

    let mut x_sort = ps.to_vec();
    let mut y_sort = ps.to_vec();

    x_sort.sort_by_key(|p| Total(p.0));
    y_sort.sort_by_key(|p| Total(p.1));
    rec(&x_sort, &y_sort)
}

/// Is line a-b and line c-d intersected ?
#[snippet = "is_intersected"]
pub fn is_intersected(a: Vector2D, b: Vector2D, c: Vector2D, d: Vector2D) -> bool {
    let ta = (c.0 - d.0) * (a.1 - c.1) + (c.1 - d.1) * (c.0 - a.0);
    let tb = (c.0 - d.0) * (b.1 - c.1) + (c.1 - d.1) * (c.0 - b.0);
    let tc = (a.0 - b.0) * (c.1 - a.1) + (a.1 - b.1) * (a.0 - c.0);
    let td = (a.0 - b.0) * (d.1 - a.1) + (a.1 - b.1) * (a.0 - d.0);

    tc * td <= 0.0 && ta * tb <= 0.0
    // Not intersects start or end point.
    // tc * td < 0.0 && ta * tb < 0.0
}

#[test]
fn test_convex_hull() {
    let vs = vec![
        Vector2D(-1.0, -1.0),
        Vector2D(-1.0, 1.0),
        Vector2D(1.0, 1.0),
        Vector2D(1.0, -1.0),
        Vector2D(0.0, 0.0),
        Vector2D(0.1, 0.1),
    ];

    let mut idx = convex_hull(&vs);
    idx.sort();

    assert_eq!(&idx, &[0, 1, 2, 3]);
}

#[snippet = "Line2D"]
#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    p: Vector2D,
    d: Vector2D,
}
#[snippet = "Line2D"]
impl Line2D {
    pub fn intersection(a: Line2D, b: Line2D) -> Vector2D {
        let n = b.d.normal();
        dbg!(n);
        let x = n.dot(b.p - a.p) / n.dot(a.d);
        a.p + a.d * x
    }
    pub fn distance(self, a: Vector2D) -> f64 {
        let perpendicular = Line2D {
            p: a,
            d: self.d.unit(),
        };
        let q = Self::intersection(self, perpendicular);
        (a-q).len()
    }
}

#[test]
fn test_intersection() {
    let m = Line2D { p: Vector2D(0.,0.), d: Vector2D(1.,1.) };
    let l1 = Line2D { p: Vector2D(0.,2.), d: Vector2D(1.,0.) };
    let l2 = Line2D { p: Vector2D(0.,2.), d: Vector2D(1.,-1.) };
    let p1 = Line2D::intersection(m, l1);
    let p2 = Line2D::intersection(m, l2);
    assert_eq!(p1, Vector2D(2.,2.));
    assert_eq!(p2, Vector2D(1.,1.));
}