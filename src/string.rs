pub type StringBuf<Marker> = crate::NewtypeBuf<Marker, String>;
pub type StringRef<Marker> = crate::NewtypeRef<Marker, String>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_newtype() {
        enum S {}
        type SBuf = StringBuf<S>;
        type SRef = StringRef<S>;

        let s: SBuf = "Hello".into();
        let s_ref: &SRef = &s;
        let hello: &SRef = "Hello".into();
        assert_eq!(hello, s_ref);
    }
}
