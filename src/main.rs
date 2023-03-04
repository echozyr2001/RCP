use cp_core::*;
use cp_view::*;

fn main() {
    let text = "'b''c''' '\n'".to_string();
    lexical_analyzer::analyze(&text);
    show();
}
