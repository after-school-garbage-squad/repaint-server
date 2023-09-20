#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![recursion_limit = "1000"]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod warp_path;

#[proc_macro_error]
#[proc_macro]
pub fn wpath(input: TokenStream) -> TokenStream {
    warp_path::warp_path(input)
}
