fn main() {
    let name = String::from("SkyGuy");
    let health = 100;
    print_name(name);
    print_health(health);

    //This line will cause an error because the ownership of the variable name has been moved to the print_name
    //function, because String is not a primitive data type and it does not implement the Copy trait.
    //print_name(name);

    //This line will work because the ownership of the variable health does not move to
    //the print_health function because it is a primitive data type and it implements the Copy trait.
    print_health(health);
}

fn print_name(x: String) {
    println!("The username is: {x}");
}

fn print_health(h: i32) {
    println!("The health is: {h}");
}
