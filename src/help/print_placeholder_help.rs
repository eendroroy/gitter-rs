use crate::gitter::cli::CLAP_STYLE;

pub fn print_placeholder_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();
    let placeholder = CLAP_STYLE.get_placeholder();

    placeholder_template!(header, usage, literal, placeholder, 20);
}
