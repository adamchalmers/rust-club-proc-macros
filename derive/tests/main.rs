use rust_club_proc_macros::ListOfValues;

#[derive(ListOfValues)]
pub struct Person {
    age: u8,
    first_name: String,
    last_name: String,
    height_cm: u8,
}
