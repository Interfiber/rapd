This guide can help you get rapd autostarting with you're window manager or desktop environment

## Auto starting with bspwm
You can auto-start rapd easily in bspwm by making it execute in you're bspwmrc like below:
```bash
# .. Snip ..
rapd & # Notice the & sign, this will detatch rapd from the process
```

## Auto starting with awesomewm
You can auto-start rapd in awesomewm using the [awful.spawn.with_shell](https://awesomewm.org/apidoc/libraries/awful.spawn.html#awful.spawn.with_shell).
Add the following line to youre ```rc.lua``` to auto start it.
```lua
-- .. Snip ..
awful.spawn.with_shell("rapd")
```

## Auto starting with kde
Kde has a guide on how to create login scripts, simply create one to run the ```rapd``` command. You can find the guide [here](https://userbase.kde.org/System_Settings/Autostart)

## Auto starting in macOS
MacOS has support for launch daemons, you can create one of these to auto start rapd on system boot or user login.
You can find the docs for them [here](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html).

## Auto starting with any X window manager or desktop environment
If you're window manager or desktop environment is not listed here you can add a new line to you're ```~/.xinitrc``` that makes rapd start on login.
Note that this might not work if you use a display manager. Open up you're .xinitrc and add the following line **BEFORE** you spawn in you're window manager
```bash
# .. Snip ..
rapd & # Make sure to include the & sign
```


