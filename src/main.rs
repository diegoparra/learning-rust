fn main() {
    /*
        Creating a empty string using the method new
    */
    let mut name_empty = String::new();
    // It's possible to append to this variable using the method push_str
    name_empty.push_str("d");
    name_empty.push_str("i");
    name_empty.push_str("e");
    name_empty.push_str("g");
    name_empty.push_str("o");
    println!("{}", name_empty);

    /*
       It's possible to create a String directly at the end adding the method to_string
    */
    let name_from_method = "diego".to_string();
    println!("{}", name_from_method);

    /*
        Creating string from slice of chars
        using the method from_iter
    */
    let chars = ['d', 'i', 'e', 'g', 'o'];
    let name_one = String::from_iter(chars);
    println!("{}", name_one);

    /*
       Creating a variable a passing into a String using the method .into() at the end
    */

    let name_into: String = "Diego".into();
    println!("{}", name_into);
}
