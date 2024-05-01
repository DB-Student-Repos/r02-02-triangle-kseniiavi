pub struct Triangle{
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
            let[a, b, c] = sides;
            if a + b > c && a + c > b && b + c > a && a > 0 && b > 0 && c > 0 {
                Some(Triangle { sides })
            }else {
                None
            }

        //unimplemented!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c

        //unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a!=b && a!=c && b!=c


        //unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a==b || a==c ||b==c


        //unimplemented!("Determine if the Triangle is isosceles.");
    }
}
