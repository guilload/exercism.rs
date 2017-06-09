use TriangleClass::*;


#[derive(Debug, PartialEq)]
enum TriangleClass {
    Equilateral,
    Isosceles,
    Scalene,
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    class: TriangleClass,
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {

    pub fn build(sides: [u32; 3]) -> Result<Triangle, ()> {
        let mut abc = sides.clone();
        abc.sort();

        let a = abc[0];
        let b = abc[1];
        let c = abc[2];

        if a + b < c || !sides.iter().all(|&s| s > 0) {
            Err(())
        }
        else {
            let class = if a == b && b == c { Equilateral }
                        else if a == b || b == c { Isosceles }
                        else { Scalene };

            Ok(Triangle { class: class, a: a, b: b, c: c })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.class == Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        self.class == Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        self.class == Scalene
    }

}
