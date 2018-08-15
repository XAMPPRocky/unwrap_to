# unwrap_to

[![Build status](https://img.shields.io/travis/Aaronepower/unwrap_to.svg?branch=master)](https://travis-ci.org/Aaronepower/unwrap_to)
[![](https://tokei.rs/b1/github/Aaronepower/unwrap_to?category=code)](https://github.com/Aaronepower/tokei)
[![Documentation](https://docs.rs/tokei/badge.svg)](https://docs.rs/tokei/)
[![Donate using Liberapay](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/Aaronepower/donate)


A simple utility macro that allows you to easily unwrap a ADT(Algebraic
Data Type) enum into a reference of its inner type.
```
#[macro_use] extern crate unwrap_to;
//!
enum Rule {
    String(String),
    Number(u64),
}
//!
fn main() {
    let rule = Rule::Number(7);
    assert_eq!(&7, unwrap_to!(rule => Rule::Number));
}
```
