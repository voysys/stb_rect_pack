use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum PackError {
    #[error("No input given")]
    NoInput,
    #[error("Unexpected input dimension")]
    Dimension(String),
    #[error("Failed to pack")]
    FailedToPack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub width: i32,
    pub height: i32,
    pub packed_top_left_x: i32,
    pub packed_top_left_y: i32,
}

impl Rect {
    pub fn new(width: i32, height: i32) -> Rect {
        Self {
            width,
            height,
            packed_top_left_x: 0,
            packed_top_left_y: 0,
        }
    }
}

/// Pack rects into width x height output rect.
/// See [stb_rect_pack](https://github.com/nothings/stb/blob/master/stb_rect_pack.h) for algorithm details.
pub fn pack(rects: &mut [Rect], width: i32, height: i32) -> Result<(), Box<dyn Error>> {
    if rects.is_empty() {
        Err(PackError::NoInput)?
    }

    let mut stb_rects = Vec::new();

    for (i, rect) in rects.iter().enumerate() {
        if rect.width <= 0 || rect.height <= 0 {
            Err(PackError::Dimension("Zero-sized input rect".to_owned()))?
        }
        stb_rects.push(stb_rect_pack_sys::stbrp_rect {
            id: i as i32,
            w: rect.width,
            h: rect.height,
            x: 0,
            y: 0,
            was_packed: 0,
        });
    }

    let mut context = stb_rect_pack_sys::stbrp_context::default();
    let mut tmp_nodes: Vec<stb_rect_pack_sys::stbrp_node> =
        vec![stb_rect_pack_sys::stbrp_node::default(); 4 * rects.len()];

    unsafe {
        stb_rect_pack_sys::stbrp_init_target(
            &mut context as *mut _,
            width,
            height,
            tmp_nodes.as_mut_ptr(),
            tmp_nodes.len() as _,
        )
    }

    let pack_result = unsafe {
        stb_rect_pack_sys::stbrp_pack_rects(
            &mut context as *mut _,
            stb_rects.as_mut_ptr(),
            stb_rects.len() as i32,
        )
    };

    if pack_result != 1 {
        Err(PackError::FailedToPack)?
    }

    'outer: for (i, rect) in rects.iter_mut().enumerate() {
        for stb_rect in &stb_rects {
            if stb_rect.id == i as i32 {
                rect.packed_top_left_x = stb_rect.x;
                rect.packed_top_left_y = stb_rect.y;
                continue 'outer;
            }
        }
        Err("Internal error")?
    }

    Ok(())
}

#[test]
fn test_basic_packing() {
    for _ in 0..10 {
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
    }
}
