# rTUI 

An attempt to create a simple tool for build Textual User Interface outside of the terminal. 

The main purpose for this is to build debugging tool and utility developpement. 

I want them outside of the terminal to allow me to and more rich feature in a near futur like image or event video. 

But we start with the basic : a Textual only User interface

More on usage and stuff later.

# screenshot

```rust
let mut screen = view::Screen::new(WIDTH,HEIGHT);
screen.draw_at("0123456789\nABCEDFGHIJKLMNOPQRSTUVWXYZ\n",0,0);
```

[screen 0](screenshot/screen_0.png)