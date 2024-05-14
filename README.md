This is a part of RustyBun a Cipboard application written for ubuntu.

This part compiles a rust application that reads the content from terh clipboard and saves it as a json.

I will create a Node.js addons in Rust from this and use it in an electron application to create a clipboard application that will behave in a way similar to windows clipboard.

Uses x11_clipboard, serde, serde_json external crates.
