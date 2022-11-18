# pbsys

#### Intro
rust powerbuilder system Lib

#### Useage

RUST：
```rust
#[no_mangle]
pub extern "stdcall" fn rust_bit_or(obthis:ObVm,nargs:u32)->i32{

    let arg1 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let arg2 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let _ = obthis.set_return_long(arg1 | arg2);
    return 1;
}
```
POWERBUIDER：
```vbscript
function long bit_or(readonly long a,readonly long b) system library "tpsys.dll" alias for "rust_bit_or"
```
