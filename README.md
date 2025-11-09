This is a gpu-screen-recorder script to save replay buffers for Hyprland. You will have to have a global bind set
in your hyprland.conf.

Examples:
```
    bind = SUPER, F10, pass, screen_record, save
    bind = SUPER, F7,  pass, screen_record, kill
```

You will also have to set the directory you want recordings to save into in main.rs.

I don't program. This is my first script ever made in rust. If you don't like something make a pr, otherwise eat a dick because
im trill as fuck.

I haven't tested the kill keybind for hyprland and it prolly wont kill the process. I'll fix that later
