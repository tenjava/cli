use std::io;
use serde_json;

error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    // It is also possible to leave this block out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    // Automatic conversions between this error chain and other
    // error chains. In this case, it will e.g. generate an
    // `ErrorKind` variant called `Dist` which in turn contains
    // the `rustup_dist::ErrorKind`, with conversions from
    // `rustup_dist::Error`.
    //
    // This section can be empty.
    links {
        // rustup_dist::Error, rustup_dist::ErrorKind, Dist;
        // rustup_utils::Error, rustup_utils::ErrorKind, Utils;
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in this case, the
    // `ErrorKind::Temp` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // This section can be empty.
    foreign_links {
        io::Error, IoError;
        serde_json::error::Error, JsonError;
        // temp::Error, Temp;
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
        // InvalidToolchainName(t: String) {
        //     description("invalid toolchain name")
        //     display("invalid toolchain name: '{}'", t)
        // }
    }
}
