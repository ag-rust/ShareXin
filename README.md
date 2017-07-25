# ShareXin  

![](https://img.shields.io/crates/v/sharexin.svg)

![](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui.png)

## Requirements
* gnome-screenshot (for now, next release switching to maim)  
* [t](https://github.com/sferik/t) (for now)  
* [toot](https://github.com/ihabunek/toot) (for now)  

## Dependencies for compiling
* gtk3   
* cairo  
* libnotify  
* pango  
* gdk-pixbuf2
* atk

#### Compiled binary hasn't been completely tested  

## Features
* Uploads to Twitter and Mastodon  
* Allows taking screenshots and saving them to files  
* Notifications via libnotify  
* Works with Wayland (On Gnome only) and X11 (On everything since the 90s)  
* Works with anything GTK has support for (including Mac and Windows with a bit of hacking)  
* Saves screenshots to folder in Pictures dir  

## Installation (via Github)  
1. `git clone https://github.com/ShareXin/`  
2. Mac Users: `brew install gtk+3`  
2. `cargo install`  
3. Login to Twitter and/or Mastodon using t and/or toot  
4. Explore `--help`  

## Installation (via Crates.io)  
1. Mac Users: `brew install gtk+3`  
1. `cargo install sharexin`  
2. Login to Twitter and/or Mastodon using t and/or toot  
3. Explore `--help`  

## Known issues  
#### If `t` won't send your tweet  
Check your $PATH. Your terminal may be able to launch it, but your WM/DE may not.  
If you have a WM like i3, you can add it to your PATH in your `.xprofile`.  

#### If `sharexin` isn't found  
Add `$HOME/.cargo/bin` to your `$PATH`.  

#### If `t` takes forever to send a tweet  
Remember that it's only a Ruby app...  

#### No command line parameters work on Mac
I can't test Mac at the moment, all I know is that it compiles.  

## Changelog  
#### [0.2.7] - 2017-07-24
- Notifications via libnotify (bye bye dbus)
- Username gotten by $USER var, rather than an entire library (thanks std!)

#### [0.2.6] - 2017-07-23  
- Forgot to update the version # for 0.2.5 from 0.2.4 and Crates wouldn't allow a reupload so....  

#### [0.2.5] - 2017-07-23  
- Better word wrap  
- Better temp dir  
- Notification actually shows image now  

#### [0.2.4] - 2017-07-21  
- Added version info  
- Made --help prettier  

#### [0.2.3] - 2017-07-21  
- Send button now says Toot or Tweet depending on where you're going  
- TextView no longer accepts tabs  

#### [0.2.2] - 2017-07-21  
- TextView now word wraps  
- Ability to simply just tweet without an image  
- Mort  

#### [0.2.1] - 2017-07-20  
- Centered window (why isn't .set_position() IN THE DOCS)  

#### [0.2.0] - 2017-07-20  
- Uh, if you're haven problems with t not loadin', check your $PATH  

#### [0.1.0] - 2017-07-19  
- Bug fixes and improvements  

#### [0.0.0] - 2017-07-19  
##### Added  
- First commit  
