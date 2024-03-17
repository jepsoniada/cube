use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::Map;
use serde::{Serialize, Deserialize};
use surrealdb::Surreal;
use surrealdb::sql::{Thing, Object};
use surrealdb::engine::local::Mem;

#[derive(Debug, Copy, Clone, PartialEq)]
enum RotDirection {
    Right,
    Left
}

trait Rotations {
    // fn rotate(A) -> A
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
enum EdgeRot {
    Correct,
    Incorrect
}
impl Rotations for EdgeRot {}
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
enum CornerRot {
    YFacing,
    XFacing,
    ZFacing
}
impl Rotations for CornerRot {}

trait Piece {
    fn rotate(&self, dir: RotDirection) -> Self;
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
struct Edge(EdgeRot, EdgeNotation);
impl Piece for Edge {
    fn rotate(&self, dir: RotDirection) -> Edge {
	match self {
	    Edge(EdgeRot::Correct, a) => Edge(EdgeRot::Incorrect, *a),
	    Edge(EdgeRot::Incorrect, a) => Edge(EdgeRot::Correct, *a),
	}
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
enum EdgeNotation {
    I, J, K, L, M, N, O, P, Q, R, S, T
}
impl EdgeNotation {
    fn variants() -> [EdgeNotation; 12] {
	use EdgeNotation::*;
	[I, J, K, L, M, N, O, P, Q, R, S, T]
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
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
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
#[derive(Debug, Copy, Clone, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
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
    fn pair_swap(&mut self, a: CornerComutation, b: CornerComutation, c: EdgeComutation, d: EdgeComutation) -> &Self {
	vec![a, b].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.corners[x.0.index()].rotate(rot); ()
	    }
	});
	vec![c, d].iter().for_each(|&x| {
	    if let Some(rot) = x.1 {
		self.edges[x.0.index()].rotate(rot); ()
	    }
	});
	self.corners.swap(a.0.index(), b.0.index());
	self.edges.swap(c.0.index(), d.0.index());
	&*self
    }
    // TODO
    fn r#move(&self, motion: MoveNotation) {}
}

#[derive(Serialize)]
struct TestCube<'a> {
    corners: [&'a str; 8],
    edges: [&'a str; 12],
}

#[derive(Deserialize)]
struct Record {
    id: Thing
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    use CornerNotation::*;
    use EdgeNotation::*;
    use RotDirection::*;

    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("test").use_db("test").await?;

    db.create::<Vec<Record>>("cube")
	.content(Cube::new())
	.await?;
    
    let res = db
	.query("select * from cube")
	.await?;
    
    print!("{res:?}");
    
    Ok(())
}
