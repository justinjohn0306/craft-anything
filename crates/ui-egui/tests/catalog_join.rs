//! The build script joins each language's catalog folder (`build.rs`), so a change that adds
//! strings can put them in a file of its own instead of appending to one shared file, where any
//! two such changes conflicted (#1465).

#[allow(dead_code)]
#[path = "../build.rs"]
mod build;

#[test]
fn a_languages_catalog_files_are_joined_in_name_order() {
    let src = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(src.path().join("de")).unwrap();
    // The last line of a file without a line break must not run into the next file's first.
    std::fs::write(src.path().join("de/r5.4-budget.tsv"), "Budget\tBudget").unwrap();
    std::fs::write(src.path().join("de/core.tsv"), "# de\nSettings\tEinstellungen\n").unwrap();
    std::fs::write(src.path().join("de/notes.txt"), "not a catalog\tnope\n").unwrap();
    std::fs::write(src.path().join("awaiting.txt"), "Budget\n").unwrap();
    build::join_catalogs(src.path(), out.path());
    let joined = std::fs::read_to_string(out.path().join("de.tsv")).unwrap();
    assert_eq!(joined, "# de\nSettings\tEinstellungen\nBudget\tBudget\n");
    assert!(!out.path().join("awaiting.tsv").exists(), "only folders are catalogs");
}
