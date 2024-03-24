pub mod second_level_mod;

pub fn println_a_to_z() {
    for c in 'a'..= 'Z' {
        println!("{}", c);
    }
}