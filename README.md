# (R)ust (F)ile (M)anager

## Rust powered terminal file-manager; inspired by ViFM and Ranger

# Currently in development!!

# KeyBindings ( stealing a lot from Dired )
    - "a" add file or directory ( end with / )
    - "x" mark for deletion
    - "u" unmark highlighted item
    - "U" unmark all
    - "V" multi highlight
    - "h" help menu
    - "c" copy
    - "p" paste
    - "l" toggle more info ( permissions, owner, size?, )
    - "Tab" tree view of highlighed directory
    - "Enter" enter directory OR open file in preview (split)

    - "-" Up directory ( cd .. )
    - "Shift-Enter" Down directory ( cd directory )
        - ? Open file in floating window using cat - would be pretty cool


## Cool Feature Ideas:
   - Keep track of working directories ( kinda like harpoon nvim )
   - Preview images
   - Filetype specific highlighting


# Links
[https://github.com/vifm/vifm]
[https://github.com/ranger/ranger]
[https://github.com/fdehau/tui-rs]
[https://github.com/crossterm-rs/crossterm]
