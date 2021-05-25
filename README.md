<!-- cargo-sync-readme start -->

# 🕊️ Palombe [![cargo version](https://img.shields.io/crates/v/palombe.svg)](https://crates.io/crates/palombe)

Palombe lets you send and receive key/value messages synchronously through
different processes using named pipes.

## Quick example

```rust
extern create palombe;

fn main() {
    std::thread::spawn(|| send("foo", "bar"));
    assert_eq!(receive("foo"), "bar");
}
```

Aknowlegments
-------------

:warning: This tool is not suited for building soytware, it is intend to
be use only in rapid prototyping and first product development steps!

C-bindings that expose Palombe have no UTF8 support (because it uses
`CString` that are FFI-Safe), so `base64` could be a good encoding for
sharing complex datatypes ...

If you looking for a better / faster / saffer way to share typed (yes
you want that) data across different processes, take a look at
[GoogleProtocal Buffer](https://developers.google.com/protocol-buffers/) or
even better at [Cap’n Proto](https://capnproto.org/) (which is
infinitely faster).

Supported environments
----------------------

The tool is embed into modules targeting several environments:

-   ECMAScript: [npm](https://www.npmjs.com/package/palombe) \|
    [Yarn](https://yarnpkg.com/fr/package/palombe)
    ([Sources](https://github.com/yvan-sraka/palombe-node))
-   Python: [PyPI](https://pypi.org/project/palombe/)
    ([Sources](https://github.com/yvan-sraka/palombe-python))
-   Ruby: [RubyGem.org](https://rubygems.org/gems/palombe)
    ([Sources](https://github.com/yvan-sraka/palombe-ruby))
-   Rust: [Crates.io](https://crates.io/crates/palombe)
    ([Sources](https://github.com/yvan-sraka/palombe-rust))

Contributing
------------

Please read
[CONTRIBUTING.md](https://github.com/yvan-sraka/Palombe/blob/master/CONTRIBUTING.md)
for details on our code of conduct, and the process for submitting pull
requests to us.

Authors
-------

-   [Yvan Sraka](https://github.com/yvan-sraka)

See also the list of
[contributors](https://github.com/yvan-sraka/Palombe/graphs/contributors)
who participated in this project.

License
-------

This project is licensed under the 3rd version of GPL License - see the
[LICENSE](https://github.com/yvan-sraka/Palombe/blob/master/LICENSE)
file for details.

<!-- cargo-sync-readme end -->