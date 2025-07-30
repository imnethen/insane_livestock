use bevy::prelude::*;

pub fn keycode_to_string(code: &KeyCode) -> Result<&str, ()> {
    match code {
        KeyCode::KeyA => Ok("a"),
        KeyCode::KeyB => Ok("b"),
        KeyCode::KeyC => Ok("c"),
        KeyCode::KeyD => Ok("d"),
        KeyCode::KeyE => Ok("e"),
        KeyCode::KeyF => Ok("f"),
        KeyCode::KeyG => Ok("g"),
        KeyCode::KeyH => Ok("h"),
        KeyCode::KeyI => Ok("i"),
        KeyCode::KeyJ => Ok("j"),
        KeyCode::KeyK => Ok("k"),
        KeyCode::KeyL => Ok("l"),
        KeyCode::KeyM => Ok("m"),
        KeyCode::KeyN => Ok("n"),
        KeyCode::KeyO => Ok("o"),
        KeyCode::KeyP => Ok("p"),
        KeyCode::KeyQ => Ok("q"),
        KeyCode::KeyR => Ok("r"),
        KeyCode::KeyS => Ok("s"),
        KeyCode::KeyT => Ok("t"),
        KeyCode::KeyU => Ok("u"),
        KeyCode::KeyV => Ok("v"),
        KeyCode::KeyW => Ok("w"),
        KeyCode::KeyX => Ok("x"),
        KeyCode::KeyY => Ok("y"),
        KeyCode::KeyZ => Ok("z"),
        KeyCode::Space => Ok(" "),
        KeyCode::Digit1 => Ok("1"),
        KeyCode::Digit2 => Ok("2"),
        KeyCode::Digit3 => Ok("3"),
        KeyCode::Digit4 => Ok("4"),
        KeyCode::Digit5 => Ok("5"),
        KeyCode::Digit6 => Ok("6"),
        KeyCode::Digit7 => Ok("7"),
        KeyCode::Digit8 => Ok("8"),
        KeyCode::Digit9 => Ok("9"),
        KeyCode::Digit0 => Ok("0"),
        KeyCode::Minus => Ok("_"),
        _ => Err(()),
    }
}

pub fn explosion(asset_handles: &Res<crate::AssetHandles>, pos: Vec3) -> impl Bundle {
    (
        AudioPlayer::new(asset_handles.explosion_sound.clone().unwrap()),
        PlaybackSettings::DESPAWN
            .with_volume(bevy::audio::Volume::Linear(0.5))
            .with_spatial(true)
            .with_spatial_scale(bevy::audio::SpatialScale::new(0.03)),
        Transform::default().with_translation(pos),
        GlobalTransform::default(),
        Mesh3d(asset_handles.explosion_cube.clone().unwrap()),
        MeshMaterial3d(asset_handles.explosion_material.clone().unwrap()),
    )
}
