This is a part of RustyBun a Cipboard application written for ubuntu.

This part compiles a rust application that reads the content from terh clipboard and saves it as a json.

I will create a Node.js addons in Rust from this and use it in an electron application to create a clipboard application that will behave in a way similar to windows clipboard.

Uses x11_clipboard, serde, serde_json external crates.


to create a node module using Neon 
npm init neon hello-word to create the neon project
Copy the code properly from main.rs to lib.rs 
Will add a copy of lib.rs
install npm install -g neon-cli

Build using npm run build
