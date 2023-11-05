# auto_undiscord
Did you hear that Discord will be blocking external websites from using images hosted on their servers?
Did you host every image on your website on Discord and now don't want to change hundreds of image links?


This program automatically scans a website's sourcecode for Discord CDN links and downloads every image into the public folder. It also updates the original links to reference the freshly downloaded files!

## How to use
Make sure Rust and Cargo are installed:
```bash
git clone https://github.com/Dot32IsCool/auto_undiscord.git
cd auto_undiscord
cargo run
```
After the program starts, you must type in the relative path to the folder of the website you would like to automatically update. Auto undiscord will take care of the rest!
