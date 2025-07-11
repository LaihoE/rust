use super::*;
use crate::create_default_session_globals_then;

#[test]
fn interner_tests() {
    let i = Interner::prefill(&[], &[]);
    // first one is zero:
    assert_eq!(i.intern_str("dog"), Symbol::new(0));
    // re-use gets the same entry, even with a `ByteSymbol`
    assert_eq!(i.intern_byte_str(b"dog"), ByteSymbol::new(0));
    // different string gets a different #:
    assert_eq!(i.intern_byte_str(b"cat"), ByteSymbol::new(1));
    assert_eq!(i.intern_str("cat"), Symbol::new(1));
    // dog is still at zero
    assert_eq!(i.intern_str("dog"), Symbol::new(0));
}

#[test]
fn without_first_quote_test() {
    create_default_session_globals_then(|| {
        let i = Ident::from_str("'break");
        assert_eq!(i.without_first_quote().name, kw::Break);
    });
}
