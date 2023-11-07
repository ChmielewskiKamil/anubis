/// 1 unused parameter: b
fn foo(a: int, b: int) -> int {
    return a + a;
}

/// 1 unused parameter: b
fn bar(a: int, b: int, c: bool) -> int {
    if c {
        return a + a;
    }
}

/// Total unused parameters in this file: 2
