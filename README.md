# Auto Undiscord
Did you hear that Discord will be blocking external websites from using images hosted on their servers?
Did you host every image on your website on Discord and now don't want to change hundreds of image links?

This program automatically scans a website's sourcecode for Discord CDN links and downloads every image into the public folder. It also updates the original links to reference the freshly downloaded files!

<img width="1370" alt="Screenshot 2023-11-17 at 14 24 43" src="https://github.com/Dot32IsCool/auto_undiscord/assets/61964090/02aecaff-ae40-46e5-b884-2cda19918aaa">

## How to use
Make sure Rust and Cargo are installed:
```bash
git clone https://github.com/Dot32IsCool/auto_undiscord.git
cd auto_undiscord
cargo run
```
After the program starts, you must type in the relative path to the folder of the website you would like to automatically update. Auto undiscord will take care of the rest!

## Caveats 
- The program downloads images to public/images, and replaces CDN links with /images. If you don't use a web framework, this will be incorrect.
- Make sure the public/images directory exists, or the links will be modified without downloading them anywhere.
- If the folder you select doesn't exist, there will be no error message, rather the program will just tell you it updated 0 links

It is recommended to use Git or keep a backup incase something goes wrong. If you want any changes made to the program, join my [Discord Server](https://discord.gg/Pswb8khdgQ) and ping me (@Dot32)
