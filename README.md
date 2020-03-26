> declare what dir you want and we do next for you
# what it is
a declarative fs lib for rust
# example
```rust
let tmp_dir = TempDir::new("example").unwrap();
let home = tmp_dir.path();
let root = home.join("a");

declare_dir(
    &root,
    vec![
        FsItem::Dir("1".to_string()),
        FsItem::StringFile("1.txt".to_string(), "ssssss".to_string()),
    ],
)
.unwrap();

let unzip_dir = home.join("a-1/a");
let zip_path = home.join("a.zip");

zip_dir(&root, &zip_path).unwrap();
unzip(&zip_path, &unzip_dir).unwrap();

let ret = dir_eq(&unzip_dir, &root).unwrap();
assert_eq!(ret, true);
```