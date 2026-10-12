fn main() {

    let d_num: i32 = 2_000;
    let i_num = d_num as i16;

    let num_array: [i8; 5] = [6, 9, 20, 25, 43];

    let hete_tuple = (true, 4, "friends");

    let deci_num = 3.04578457;

    let with_milk = true;
    let with_sugar = true;

    let is_my_type_of_coffee = with_milk & with_sugar;

    let is_acceptable_coffee = with_milk | with_sugar;

    println!("{:.3}", deci_num);


    println!("{hete_tuple:?}");
    println!("{num_array:?}");
    println!("{is_my_type_of_coffee}");
    println!("{is_acceptable_coffee}");
}
