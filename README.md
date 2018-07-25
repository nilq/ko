# Ko 🐄

A configuration language for electronic computers; for embedded use.

## Syntax

```moon
title = "
  Just remember:
  what you’re seeing and what you're reading is not what’s happening.
"

window:
  width  = 400 + 400
  height = 600

  -- I really doubt your config language can do this
  title  = "Window: " + width + "x" + height

server:
  name = "bob"
  ip   = "10.0.0.1"

  server.client:
    name = "lisa"
    favorite_numbers = [1, 2, 3]
```

## License

MIT License, whatever
