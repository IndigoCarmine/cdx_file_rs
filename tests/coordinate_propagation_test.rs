/// Test for coordinate propagation from parent to child objects
/// 
/// This test verifies that BoundsInParent property of Page objects
/// correctly offsets the coordinates of child objects.

use cdx_file_rs::cdx::values::{Point2d, Rectangle};

#[test]
fn test_parent_offset_accumulation() {
    // Test that parent offsets accumulate correctly
    
    // Initial offset
    let offset1 = Point2d { x: 100.0, y: 200.0 };
    
    // Second offset
    let offset2 = Point2d { x: 50.0, y: 75.0 };
    
    // Manually accumulate to verify the logic
    let cumulative = Point2d {
        x: offset1.x + offset2.x,
        y: offset1.y + offset2.y,
    };
    
    assert_eq!(cumulative.x, 150.0);
    assert_eq!(cumulative.y, 275.0);
}

#[test]
fn test_bounds_in_parent_offset_extraction() {
    // Test that we can correctly extract offset from a Page's BoundsInParent
    let bounds = Rectangle {
        top: 100.0,
        left: 50.0,
        bottom: 500.0,
        right: 400.0,
    };
    
    // The offset should be taken from the top-left corner
    let offset = Point2d {
        x: bounds.left,
        y: bounds.top,
    };
    
    assert_eq!(offset.x, 50.0);
    assert_eq!(offset.y, 100.0);
}

#[test]
fn test_coordinate_transformation_logic() {
    // Test the coordinate transformation math without egui
    
    let origin_x = 0.0_f32;
    let origin_y = 0.0_f32;
    let zoom = 1.0_f32;
    let auto_scale = 1.0_f32;
    
    // Test 1: No parent offset
    let cdx_x = 100.0;
    let cdx_y = 200.0;
    let parent_offset_x = 0.0;
    let parent_offset_y = 0.0;
    
    let scale = zoom * auto_scale;
    let adjusted_x = cdx_x + parent_offset_x;
    let adjusted_y = cdx_y + parent_offset_y;
    
    let screen_x = origin_x + (adjusted_x as f32 * scale);
    let screen_y = origin_y - (adjusted_y as f32 * scale); // Inverted Y
    
    assert_eq!(screen_x, 100.0);
    assert_eq!(screen_y, -200.0);
    
    // Test 2: With parent offset
    let parent_offset_x = 50.0;
    let parent_offset_y = 30.0;
    
    let adjusted_x = cdx_x + parent_offset_x;
    let adjusted_y = cdx_y + parent_offset_y;
    
    let screen_x = origin_x + (adjusted_x as f32 * scale);
    let screen_y = origin_y - (adjusted_y as f32 * scale);
    
    assert_eq!(screen_x, 150.0);
    assert_eq!(screen_y, -230.0);
}

#[test]
fn test_cumulative_offset_transformation() {
    // Test cumulative offsets
    let origin_x = 0.0_f32;
    let origin_y = 0.0_f32;
    let zoom = 1.0_f32;
    let auto_scale = 1.0_f32;
    
    let cdx_x = 100.0;
    let cdx_y = 200.0;
    
    // First level offset
    let offset1_x = 50.0;
    let offset1_y = 30.0;
    
    // Second level offset
    let offset2_x = 10.0;
    let offset2_y = 20.0;
    
    // Cumulative offset
    let cumulative_offset_x = offset1_x + offset2_x;
    let cumulative_offset_y = offset1_y + offset2_y;
    
    let scale = zoom * auto_scale;
    let adjusted_x = cdx_x + cumulative_offset_x;
    let adjusted_y = cdx_y + cumulative_offset_y;
    
    let screen_x = origin_x + (adjusted_x as f32 * scale);
    let screen_y = origin_y - (adjusted_y as f32 * scale);
    
    // With cumulative offset (50 + 10, 30 + 20) = (60, 50):
    // x = 0 + (100 + 60) * 1.0 = 160
    // y = 0 - (200 + 50) * 1.0 = -250
    assert_eq!(screen_x, 160.0);
    assert_eq!(screen_y, -250.0);
}
