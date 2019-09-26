#[snippet = "Range"]
struct Range {
    forward: bool,
    start: usize,
    len: usize,
}
#[snippet = "Range"]
impl Range {
    fn start(i: usize) -> Range {
        Range {
            forward: true,
            start: i,
            len: 0,
        }
    }
    fn last(i: usize) -> Range {
        Range {
            forward: false,
            start: i,
            len: 0,
        }
    }
    fn to(&self, to: usize) -> Rangee {
        if self.forward {
            self.len(to+1-self.start)
        } else {
            self.len(self.start+1-to)
        }
    }
    fn until(&self, j: usize) -> Rangee {
        if self.forward {
            self.len(j-self.start)
        } else {
            self.len(self.start-j)
        }
    }
    fn len(&self, l: usize) -> Rangee {
        if self.forward {
            Rangee {
                start: self.start,
                len: l,
            }
        } else {
            Rangee {
                start: self.start + 1 - l,
                len: l,
            }
        }
    }
}
#[snippet = "Range"]
struct Rangee {
    start: usize,
    len: usize,
}
#[snippet = "Range"]
impl Rangee {
    fn fwd(&self) -> (usize, usize) {
        (self.start, self.start+self.len)
    }
    fn rev(&self, n: usize) -> (usize, usize) {
        let start = n-self.start-self.len;
        (start, start+self.len)
    }
}

#[test]
fn test_range() {
    let r1 = Range::start(1).len(5);
    assert_eq!(r1.fwd(), (1,6));
    assert_eq!(r1.rev(9), (3,8));

    let r2 = Range::last(6).len(3);
    assert_eq!(r2.fwd(), (4,7));
    assert_eq!(r2.rev(9), (2,5));
}