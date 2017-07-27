extern crate rubigo_yes;

fn main() {
    match rubigo_yes::parse_args(std::env::args().skip(1).collect()) {
        Ok(ref req) => req.do_it(),
        Err(ref err) => err.show()
    }
}
