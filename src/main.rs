use cp_core::*;
use cp_view::*;

fn main() {
    // let text = "'b''c''' '\\n'";
    let text = "_ab ds";
    lexical_analyzer::analyze(text);
    show();
}
