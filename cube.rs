use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone, PartialEq)]
enum RotDirection {
    Right,
    Left
}

trait Rotations {
    // fn rotate(A) -> A
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum EdgeRot {
    Correct,
    Incorrect
}
impl Rotations for EdgeRot {}
#[derive(Debug, Copy, Clone, PartialEq)]
enum CornerRot {
    YFacing,
    XFacing,
    ZFacing
}
impl Rotations for CornerRot {}

trait Piece {
    fn rotate(&self, dir: RotDirection) -> Self;
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Edge(EdgeRot, EdgeNotation);
impl Piece for Edge {
    fn rotate(&self, dir: RotDirection) -> Edge {
	match self {
	    Edge(EdgeRot::Correct, a) => Edge(EdgeRot::Incorrect, *a),
	    Edge(EdgeRot::Incorrect, a) => Edge(EdgeRot::Correct, *a),
	}
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Corner(CornerRot, CornerNotation);
impl Piece for Corner {
    fn rotate(&self, dir: RotDirection) -> Corner {
	match (self, dir) {
	    (Corner(CornerRot::YFacing, a), RotDirection::Right) => Corner(CornerRot::XFacing, *a),
	    (Corner(CornerRot::YFacing, a), RotDirection::Left) => Corner(CornerRot::ZFacing, *a),
	    (Corner(CornerRot::XFacing, a), RotDirection::Right) => Corner(CornerRot::ZFacing, *a),
	    (Corner(CornerRot::XFacing, a), RotDirection::Left) => Corner(CornerRot::YFacing, *a),
	    (Corner(CornerRot::ZFacing, a), RotDirection::Right) => Corner(CornerRot::YFacing, *a),
	    (Corner(CornerRot::ZFacing, a), RotDirection::Left) => Corner(CornerRot::XFacing, *a),
	}
    }
}

trait PieceNotation {
    fn index(&self) -> usize;
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum EdgeNotation {
    I, J, K, L, M, N, O, P, R, S, T, U
}
impl EdgeNotation {
    fn variants() -> [EdgeNotation; 12] {
	use EdgeNotation::*;
	[I, J, K, L, M, N, O, P, R, S, T, U]
    }
}
impl PieceNotation for EdgeNotation {
    fn index(&self) -> usize {
	*self as usize
    }
}
// impl Into<Edge> for EdgeNotation {
//     fn into(self) -> Edge {
// 	Edge(EdgeRot::Correct, self)
//     }
// }

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum CornerNotation {
    A, B, C, D, E, F, G, H
}
impl CornerNotation {
    fn variants() -> [CornerNotation; 8] {
	use CornerNotation::*;
	[A, B, C, D, E, F, G, H]
    }
}
impl PieceNotation for CornerNotation {
    fn index(&self) -> usize {
	*self as usize
    }
}
// impl Into<Corner> for CornerNotation {
//     fn into(self) -> Corner {
// 	Corner(CornerRot::YFacing, self)
//     }
// }

// macro_rules! asdf {
//     ($($a:tt) +) => { println!($($a),+) }
// }

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
enum MoveNotation {
    R, Rp, r, rp,
    U, Up, u, up,
    L, Lp, l, lp,
    D, Dp, d, dp,
    F, Fp, f, fp,
    B, Bp, b, bp,
    M, Mp,
    E, Ep,
    S, Sp,
    x, xp,
    y, yp,
    z, zp,
}

#[derive(Clone, Copy)]
struct Comutation<XN: PieceNotation>(XN, Option<RotDirection>);
type CornerComutation = Comutation<CornerNotation>;
type EdgeComutation = Comutation<EdgeNotation>;

#[derive(Debug, PartialEq)]
struct Cube {
    corners: [Corner; 8],
    edges: [Edge; 12],
}
impl Cube {
    fn new() -> Cube {
	Cube {
	    edges: EdgeNotation::variants().iter()
		.map(|&a| Edge(EdgeRot::Correct, a))
		.collect::<Vec<Edge>>().try_into().unwrap(),
	    corners: CornerNotation::variants().iter()
		.map(|&a| Corner(CornerRot::YFacing, a))
		.collect::<Vec<Corner>>().try_into().unwrap(),
	}
    }
    // TODO is comutator exhaustive???
    fn corner_swap_right(&mut self, a: CornerComutation, b: CornerComutation, c: CornerComutation) -> &Self {
	vec![a, b, c].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.corners[x.0.index()].rotate(rot); ()
	    }
	});
	self.corners.swap(a.0.index(), c.0.index());
	self.corners.swap(b.0.index(), c.0.index());
	&*self
    }
    fn corner_swap_left(&mut self, a: CornerComutation, b: CornerComutation, c: CornerComutation) -> &Self {
	vec![a, b, c].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.corners[x.0.index()].rotate(rot); ()
	    }
	});
	self.corners.swap(a.0.index(), c.0.index());
	self.corners.swap(a.0.index(), b.0.index());
	&*self
    }
    fn edge_swap_right(&mut self, a: EdgeComutation, b: EdgeComutation, c: EdgeComutation) -> &Self {
	vec![a, b, c].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.corners[x.0.index()].rotate(rot); ()
	    }
	});
	self.edges.swap(a.0.index(), c.0.index());
	self.edges.swap(b.0.index(), c.0.index());
	&*self
    }
    fn edge_swap_left(&mut self, a: EdgeComutation, b: EdgeComutation, c: EdgeComutation) -> &Self {
	vec![a, b, c].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.corners[x.0.index()].rotate(rot); ()
	    }
	});
	self.edges.swap(a.0.index(), c.0.index());
	self.edges.swap(a.0.index(), b.0.index());
	&*self
    }
    // TODO
    fn r#move(&self, motion: MoveNotation) {}
}

fn main() {
    use CornerNotation::*;
    use EdgeNotation::*;
    use RotDirection::*;
    let mut cube_a = Cube::new();
    cube_a.edge_swap_left(
	Comutation(I, None),
	Comutation(J, None),
	Comutation(K, None),
    );
    let mut cube_b = Cube::new();
    cube_b.edge_swap_right(
	Comutation(K, None),
	Comutation(J, None),
	Comutation(I, None),
    );
    assert_eq!(cube_a, cube_b)
}
