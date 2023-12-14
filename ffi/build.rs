extern crate cc;

fn main(){
    cc::Build::new().file("soma.cpp").cpp_link_stdlib("stdc++").compile("soma");
}
