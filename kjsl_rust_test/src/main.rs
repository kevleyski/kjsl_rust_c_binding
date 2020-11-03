#[link(name = "kjsl_c_lib", kind = "static")]
extern "C" {
    fn kev() -> i32;
}

fn main() {
    let retval = unsafe { kev() };
    println!("kev={}", retval);
}
