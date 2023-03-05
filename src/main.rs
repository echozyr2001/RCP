use cp_core::*;
use cp_view::*;

fn main() {
    let text = "'b''c''' '\\n'";
    lexical_analyzer::analyze(text);
    show();
}
