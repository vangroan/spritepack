error_chain! {
    // Names driven by convention.
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }

    errors {
        PackerOutOfSpace(target_slot: (u32, u32)) {
            description("packer out of space")
            display("packer out of space while packing rectangle of size: ({}, {})", target_slot.0, target_slot.1)
        }
    }
}
