//! Minimal test for macro imports

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;
use bevy_luma_widgets::luma_col;

fn main() {
    let _scene = luma_col!(
        gap: 10.0,
        children: []
    );
}