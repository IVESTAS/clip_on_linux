This is a gpu-screen-recorder script to save replay buffers for Hyprland. You will have to have a global bind set
in your hyprland.conf.

Examples:
```
    bind = SUPER, F10, pass, screen_record, save
    bind = SUPER, F7,  exec, KILL_SCREEN_RECORD_BASH_SCRIPT
```

You will also have to set the directory you want recordings to save into in main.rs.

I don't program. This is my first script ever made. If you don't like something make a pr, otherwise eat a dick because
im trill as fuck.

Idk how to kill gpu-screen-recorder on program close, so I use a bash script to kill screen_record and gpu-screen-recorder as of now. If you don't do this and keep running screen_record your computer will get
so fucking laggy lmfao. 

This isn't a good solution if you're looking for a script to record games. It has a lot of problems and is only suited for people on Hyprland.
Even with all the prerequisites, if you arent ghetto and don't have a kill script you'll get a pc going at 20 frames. 
