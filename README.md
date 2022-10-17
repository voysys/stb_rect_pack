# stb_rect_pack

Wrapper over [stb_rect_pack_sys](https://github.com/voysys/stb-rect-pack-sys), offering a cleaner abstraction than the bindgen generated interface.

[stb](https://github.com/nothings/stb)

# Example

```Rust
let mut rects = vec![Rect::new(1280, 720); 4];

pack(&mut rects, 2560, 1440).unwrap();

assert_eq!(rects[0].packed_top_left_x, 1280);
assert_eq!(rects[0].packed_top_left_y, 720);

assert_eq!(rects[1].packed_top_left_x, 0);
assert_eq!(rects[1].packed_top_left_y, 0);

assert_eq!(rects[2].packed_top_left_x, 1280);
assert_eq!(rects[2].packed_top_left_y, 0);

assert_eq!(rects[3].packed_top_left_x, 0);
assert_eq!(rects[3].packed_top_left_y, 720);
```
