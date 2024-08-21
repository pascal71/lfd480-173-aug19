// Returns the real solutions for the quadratic equation defined by aX²+bX+c
//

pub mod qeq_lib {

  use std::f64;

  pub fn solve(a: f64, b: f64, c: f64) -> (f64, f64) {

  	let discriminant = b*b - 4.0*a*c ;
	if discriminant < 0.0 {
		println!("Does not have any real solutions") ;
        return (0.0,0.0) ;
	};

	let d = discriminant.sqrt() ;
	let solution1 = (-b + d) / (2.0 * a) ;
	let solution2 = (-b - d) / (2.0 * a) ;

    (solution1, solution2)
  }
}
