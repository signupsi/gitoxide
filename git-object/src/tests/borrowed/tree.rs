mod parse {
    use crate::borrowed::tree::{Entry, Mode};
    use crate::tests::hex_to_id;
    use crate::{
        borrowed::{tree::parse, tree::Tree},
        tests::borrowed::fixture_bytes,
    };
    use bstr::ByteSlice;

    #[test]
    fn everything() {
        assert_eq!(
            parse(&fixture_bytes("tree", "everything.tree")).unwrap().1,
            Tree(vec![
                Entry {
                    mode: Mode::BlobExecutable,
                    filename: b"exe".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")[..]
                },
                Entry {
                    mode: Mode::Blob,
                    filename: b"file".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")[..]
                },
                Entry {
                    mode: Mode::Commit,
                    filename: b"grit-submodule".as_bstr(),
                    oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")[..]
                },
                Entry {
                    mode: Mode::Tree,
                    filename: b"subdir".as_bstr(),
                    oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")[..]
                },
                Entry {
                    mode: Mode::Link,
                    filename: b"symlink".as_bstr(),
                    oid: &hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e")[..]
                }
            ])
        );
    }
}