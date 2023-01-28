use std::ops::Neg;
use std::ops::MulAssign;
use std::ops::DivAssign;

// TODO:
// Maybe i did everything wrong
// Maybe I should have used operator overloading earlier
// So i dont have to use sub(vector_a, vector_b)
// Check this later.
// Continue @ 13 (34) -  Magnitude
// Should I already start creating subfiles?

fn main() {
    // Struct are like tuples, but we name each const/variable
    // My idea here was to have a struct with "default" value for w 
    // This way a point has 0.0 and vector 1.0 or whatever
    // It looks like this is not possible.
    // New Idea is to have one structure.
    // And then I will use two functions, one for point and one for vector.
    // For debugging purpose at the following code


    // I should change this to tuple instead of point, I think. lol
    #[derive(Debug, Copy, Clone)]
    pub struct Tuple {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Projectile {
        position: Tuple,
        velocity: Tuple,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Environment {
        gravity: Tuple,
        wind: Tuple,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Color {
        red: f64,
        green: f64,
        blue: f64,
    }

    // Is is good practice to use functions to create struct?
    fn create_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x: x, y: y, z: z, w: 1.0}
    }
    
    // Can I not just use vectors from rust language?
    fn create_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x: x, y:y, z: z, w: 0.0}
    }

    // What does & do? en do we need to have it mutable?
    // I guess not since we create a new struct.
    // Name standard for variables and params?
    fn add_tuples(tuple_one: Tuple, tuple_two: Tuple) -> Tuple {
        Tuple { x: tuple_one.x + tuple_two.x,
                y: tuple_one.y + tuple_two.y,
                z: tuple_one.z + tuple_two.z,
                w: tuple_one.w + tuple_two.w
        }
    }
   
    // Should check whats what with, vector, point
    fn sub_tuples(tuple_one: Tuple, tuple_two: Tuple) -> Tuple {
        Tuple { x: tuple_one.x - tuple_two.x,
                y: tuple_one.y - tuple_two.y,
                z: tuple_one.z - tuple_two.z,
                w: tuple_one.w - tuple_two.w
        }
    }

    // Create operator overload, to negate a tuple
    impl Neg for Tuple{
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
        }
    }

    // Create operator overload, for scalar
    // What is the difference between mul & MulAssign?
    // Is this the solution that I am looking for?
    // We call self.x etc here because we alter the input tuple.
    // We dont return anything but the tuple.
    impl MulAssign<f64> for Tuple{
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
            self.w *= rhs;   
        }
    } 

    impl DivAssign<f64> for Tuple{
        fn div_assign(&mut self, rhs: f64) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
            self.w /= rhs;   
        }
    }

    fn magnitude(tuple: Tuple) -> f64{
        let x = f64::powi(tuple.x, 2);
        let y = f64::powi(tuple.y, 2);
        let z = f64::powi(tuple.z, 2);
        let magnitude_v = f64::sqrt(x+y+z);
        return magnitude_v;
    }

    // Do I need to use a normal function here?
    // Check if this is the way.
    // Should I do the operation on a tuple here?
    // Do we need any restrictions on w to make sure its a vector?
    // A point is wrong I guess.
    // Maybe Print magnitude_v in magnitude function to see if the tuple has changed.
    // WHY IS THE VALUE MOVED
    // Is adding the copy trait to tuple really the solution?
    fn norm(tuple: Tuple) -> Tuple{
        //let test: Tuple = tuple;
        let magnitude_v: f64 = magnitude(tuple);
        Tuple { x: tuple.x / magnitude_v,  
                y: tuple.y / magnitude_v, 
                z: tuple.z / magnitude_v,
                w: 0.0 
            }
    }

    fn dot(tuple_a: Tuple, tuple_b: Tuple) -> f64{
        let dot_answer: f64 = (tuple_a.x * tuple_b.x) +
                              (tuple_a.y * tuple_b.y) +
                              (tuple_a.z * tuple_b.z);
        return dot_answer; 
    }

    fn cross(tuple_a: Tuple, tuple_b: Tuple) -> Tuple{
        Tuple { x: (tuple_a.y * tuple_b.z) - (tuple_a.z * tuple_b.y),
                y: (tuple_a.z * tuple_b.x) - (tuple_a.x * tuple_b.z),
                z: (tuple_a.x * tuple_b.y) - (tuple_a.y * tuple_b.x),
                w: 0.0
            }
    }
   
    fn tick(environment: Environment, mut projectile: Projectile) -> Projectile{
        let position = add_tuples(projectile.position, projectile.velocity);        
        let velocity = add_tuples(projectile.velocity,
                    add_tuples(environment.gravity, environment.wind));
        projectile.position = position;
        projectile.velocity = velocity;
        println!("I AM VELO, {:?}", projectile.velocity); 
        return projectile;
    }
        
    let point_one = create_point(4.3, -4.2, 3.1);
    let vector_one =  create_vector(4.3, -4.2, 3.1);
    
    println!("{:?}", point_one);
    println!("{:?}", vector_one);

    // Goal is to test the point structure
    // Do I need to give this structure as a param?
    // Maybe check how to properly do testing in rust.

    fn test_tuple(x: f64, y:f64, z:f64, w:f64) {
        let tuple_point = Tuple {x: x, y: y, z: z, w: w};
        assert!(tuple_point.x == x);
        assert!(tuple_point.y == y);
        assert!(tuple_point.z == z);
        if w == 1.0 {
            println!("This tuple is a point")
        } else if w == 0.0 {
            println!("This tuple is a vector")
        }
    }

    fn test_point(x: f64, y:f64, z: f64) {
        let test_point = create_point(x, y, z);
        assert!(test_point.x == x);
        assert!(test_point.y == y);
        assert!(test_point.z == z);
        assert!(test_point.w == 1.0);
    }

    fn test_vector(x: f64, y:f64, z: f64) {
        let test_vector = create_vector(x, y, z);
        assert!(test_vector.x == x);
        assert!(test_vector.y == y);
        assert!(test_vector.z == z);
        assert!(test_vector.w == 0.0);
    }

    fn test_add_tuples(x: f64, y:f64, z: f64, w: f64, a: f64, b: f64, c: f64, v: f64) {
        let xa = x + a;
        let yb = y + b;
        let zc = z + c;
        let wv = w + v;

        let tuple_a = Tuple{x: x, y: y, z: z, w: w};
        let tuple_b= Tuple{x: a, y: b, z: c, w: v};
        let tuple_ab = add_tuples(tuple_a, tuple_b);

        assert!(tuple_ab.x == xa);
        assert!(tuple_ab.y == yb);
        assert!(tuple_ab.z == zc);
        assert!(tuple_ab.w == wv);
    }

    fn test_sub_tuples(x: f64, y:f64, z: f64, w: f64, a: f64, b: f64, c: f64, v: f64) {
        let xa = x - a;
        let yb = y - b;
        let zc = z - c;
        let wv = w - v;
        println!("{} {} {} {}", xa, yb, zc, wv);

        let tuple_a = Tuple{x: x, y: y, z: z, w: w};
        let tuple_b= Tuple{x: a, y: b, z: c, w: v};
        let tuple_ab = sub_tuples(tuple_a, tuple_b);
        assert!(tuple_ab.x == xa);
        assert!(tuple_ab.y == yb);
        assert!(tuple_ab.z == zc);
        assert!(tuple_ab.w == wv);
    }

    // Test operation overload negate.
    // Here we copy the value of -tuple_a into a new tuple, is this the way to do it?
    // Seems smart so we use the same adress, otherwise we keep making copies while never using the old stuff.
    fn test_negating_tuples(x: f64, y: f64, z: f64, w: f64) {
        let tuple_a = Tuple{x: x, y: y, z: z, w: w};
        let tuple_a= -tuple_a;
        println!("{:?}", tuple_a);
        println!("{}", tuple_a.x);
        assert!(tuple_a.x == -x);
        assert!(tuple_a.y == -y);
        assert!(tuple_a.z == -z);
        assert!(tuple_a.w == -w)
    }

    // Watch how we don't redefine a variable here.
    // Since the output of scalar is the tuple itself
    // We also use self in the overload scalar. 
    fn test_scalar_tuples(x: f64, y:f64, z: f64, w: f64){
        let mut tuple_a = Tuple{x: x, y: y, z: z, w: w};
        tuple_a *= 3.0;
        println!("{:?}", tuple_a);
        assert!(tuple_a.x == 3.0 * x);
        assert!(tuple_a.y == 3.0 * y);
        assert!(tuple_a.z == 3.0 * z);
        assert!(tuple_a.w == 3.0 * w);
    }

    fn test_div_tuples(x: f64, y:f64, z: f64, w: f64){
        let mut tuple_a = Tuple{x: x, y: y, z: z, w: w};
        tuple_a /= 3.0;
        println!("{:?}", tuple_a);
        assert!(tuple_a.x == x / 3.0);
        assert!(tuple_a.y == y / 3.0);
        assert!(tuple_a.z == z / 3.0);
        assert!(tuple_a.w == w / 3.0);
    }

    fn test_mag_vector(x: f64, y:f64, z: f64){
        let vector_v = create_vector(x, y, z);
        let x = f64::powi(x, 2);
        let y = f64::powi(y, 2);
        let z = f64::powi(z, 2);
        let magnitude_v = f64::sqrt(x+y+z);
        let vector_w = magnitude(vector_v);

        assert!(vector_w == magnitude_v);
        println!("{} is {}", vector_w, magnitude_v);
    }
    
    fn test_norm_vector(x: f64, y: f64, z: f64){
        let tuple_one: Tuple = create_vector(x, y, z);
        let norm_vec = norm(tuple_one);
        let norm_x: f64 = x / magnitude(tuple_one);
        let norm_y: f64 = y / magnitude(tuple_one);
        let norm_z: f64 = z / magnitude(tuple_one);
        
        assert!(norm_x == norm_vec.x);
        assert!(norm_y == norm_vec.y);
        assert!(norm_z == norm_vec.z);

        let vec_two = create_vector(1.0, 2.0, 3.0);
        let norm_vec_two = norm(vec_two);
        println!("{:?}", norm_vec_two);

    }

    fn test_dot_product(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64){
       let vec_one = create_vector(x, y, z);
       let vec_two = create_vector(a, b, c);
        
       let dot_answer: f64 = (x * a) + (y * b) + (z * c);
       let dot_computed: f64 = dot(vec_one, vec_two);

       assert!(dot_answer == dot_computed);

       let vec_three = create_vector(1.0, 2.0, 3.0);
       let vec_four = create_vector(2.0, 3.0, 4.0);
       let dot_answer = dot(vec_three, vec_four);
       println!("{}", dot_answer);
    }

    fn test_cross_product(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64){
       let vec_one = create_vector(x, y, z);
       let vec_two = create_vector(a, b, c);
        
       let cross_answer_a: f64 = (y * c) - (z * b);
       let cross_answer_b: f64 = (z * a) - (x * c); 
       let cross_answer_c: f64 = (x * b) - (y * a);
       
       let cross_computed: Tuple = cross(vec_one, vec_two);
    
       assert!(cross_answer_a == cross_computed.x);
       assert!(cross_answer_b == cross_computed.y);
       assert!(cross_answer_c == cross_computed.z);


       let vec_three = create_vector(1.0, 2.0, 3.0);
       let vec_four = create_vector(2.0, 3.0, 4.0);
       let cross_answer = cross(vec_three, vec_four);
       println!("{:?}", cross_answer);
    }

    fn test_flight(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64){
        let gravity = create_vector(0.0, -0.1, 0.0);
        let wind = create_vector(-0.01, 0.0, 0.0);
        let position = create_point(x, y, z);
        let velocity = create_vector(a, b, c);
        let mut projectile = Projectile{position: position, velocity: velocity};         
        let environment =  Environment{gravity: gravity, wind: wind};
        //projectiles y
        let mut n = 0;
        while projectile.position.y > 0.0 {
            projectile = tick(environment, projectile);
            println!("Position:, {:?}, Amount of ticks {}", projectile.position, n);
            n += 1;
        }


    }
 
    // Should I create a function to create color?
    fn test_color(x: f64, y: f64, z: f64){
        let new_color = Color{red: x, green: y, blue: z};
        assert!(new_color.red == x);
        assert!(new_color.green == y);
        assert!(new_color.blue == z);
    }

    // Maybe here my structure fails because add_tuples is not general enough?
    fn test_add_color(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64){
        let color_one = Color{red: x, green: y, blue: z};
        let color_two = Color{red: a, green: b, blue: c};
        let color_three = add_tuples(color_one, color_two);



    }


    
    //test_tuple(2.0, 1.5, 9.9, 0.0);
    //test_point(4.0, 3.3, 2.2);
    //test_vector(2.0, 1.9, 3.3);
    //test_add_tuples(3.0, -2.0, 5.0, 1.0, -2.0, 3.0, 1.0, 0.0);
    //test_sub_tuples(3.0, 2.0, 1.0, 1.0, 5.0, 6.0, 7.0, 1.0);
    //test_sub_tuples(3.0, 2.0, 1.0, 1.0, 5.0, 6.0, 7.0, 0.0);
    //test_sub_tuples(3.0, 2.0, 1.0, 0.0, 5.0, 6.0, 7.0, 0.0);
    
    // Fun other test. create a point and vector as test and then subtract
    // Check those other vids, what did I solve in a weird way?
    //test_negating_tuples(4.0, 3.0, 2.0, 1.0);
    //test_scalar_tuples(4.0, 3.0, 2.0, 1.0);
    //test_div_tuples(4.0, 3.0, 2.0, 1.0);
    //test_mag_vector(4.0, 3.0, 2.0);
    //test_norm_vector(1.0, 2.0, 3.0);
    //test_dot_product(1.0, 2.0, 3.0, 2.0, 3.0, 4.0);
    //test_cross_product(1.0, 2.0, 3.0, 2.0, 3.0, 4.0);
    test_flight(0.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    test_color(-0.5, 0.4, 1.7);
    test_add_color(0.9, 0.6, 0.75, 0.7, 1.9, 3.2);
}