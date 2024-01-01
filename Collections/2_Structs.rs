// Declare struct
struct Point3D{
        x:i32,
        y:i32,
        z:i32
}
fn main() -> (){
    let pt = generate_point(3,4,5);
    println!("Point3D: {} {} {}" ,pt.x,pt.y,pt.z); // Access values with STRUCT_VARIABLE.STRUCT_ELEMENT => pt.x 

    // Desturcture
    // Variable names for desturcting must be SAME as struct name - aStruct is named based not order based
    let Point3D {x,y,z} = pt; 

    print!("Point3D: {} {} {}" ,x,y,z); 

}

// Function takes 3 i32's and return's a Point3D type object
fn generate_point(a:i32,b:i32,c:i32) -> Point3D {
    let point = Point3D {x:a, y:b, z:c};  // 
    return point;
}