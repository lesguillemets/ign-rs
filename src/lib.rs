use std::collections::HashMap;

pub fn ft_aliases() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("hs", "haskell"),
        ("pl", "perl"),
        ("py", "python"),
        ("rb", "ruby"),
        ("rs", "rust"),
        ("latex", "tex"),
    ])
}
