// Exercise 01 — Ownership & Borrowing
// Write your solution here

fn main() {
    let s1 = String::from("test");
    let s2 = &s1;
    println!("{} ,{}", s1, s2);
}