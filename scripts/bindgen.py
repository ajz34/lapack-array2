# # Bindgen of `lapack.h`

import subprocess

# ## Pre-process

with open("lapack.h", "r") as f:
    token = f.read()

# set lapack_int
token = token.replace("#define lapack_int        int64_t", "typedef int64_t lapack_int;")
token = token.replace("#define lapack_int        int32_t", "typedef int32_t lapack_int;")
# disable fortran strlen
token = token.replace("#define LAPACK_FORTRAN_STRLEN_END", "")

with open("lapack_bindgen.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen", "lapack_bindgen.h",
    "-o", "lapack.rs",
    "--allowlist-function", "^.*_$",
    "--use-core"
])

# ## Post-process

with open("lapack.rs", "r") as f:
    token = f.read()

# hardcode lapack_int
token = token.replace("pub type lapack_int = i32;\n", "")
token = """
#![allow(non_camel_case_types)]

use num_complex::*;

#[cfg(not(feature = "ilp64"))]
pub type lapack_int = i32;
#[cfg(feature = "ilp64")]
pub type lapack_int = i64;
""".strip() + "\n\n" + token

# __BindgenComplex<T> -> Complex<T>
token = token.replace(
    """
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}""".strip(), "")
token = token.replace("__Bindgen", "")

# change variable name to lower-case
token_list = token.split("\n")
for i in range(len(token_list)):
    l = token_list[i]
    if l.startswith("        ") and l.endswith(","):
        l_split = l.split(": ")
        assert(len(l_split) == 2)
        l_split[0] = l_split[0].lower()
        token_list[i] = ": ".join(l_split)
token = "\n".join(token_list)

with open("lapack.rs", "w") as f:
    f.write(token)

subprocess.run([
    "rustfmt", "lapack.rs"
])

subprocess.run([
    "mv", "lapack.rs", "../src/ffi"
])
