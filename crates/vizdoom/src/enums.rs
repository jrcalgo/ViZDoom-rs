//! Rust mirrors of the ViZDoom C++ enums in `include/ViZDoomTypes.h`.
//!
//! Each enum is `#[repr(i32)]` and its discriminants match the underlying C++
//! values exactly so they can be passed across the C ABI by integer cast. The
//! tests at the bottom guard the mirror against drift.

/// Engine mode. Mirrors `vizdoom::Mode`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Player = 0,
    Spectator = 1,
    AsyncPlayer = 2,
    AsyncSpectator = 3,
}

/// Screen pixel format. Mirrors `vizdoom::ScreenFormat`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenFormat {
    Crcgcb = 0,
    Rgb24 = 1,
    Rgba32 = 2,
    Argb32 = 3,
    Cbcgcr = 4,
    Bgr24 = 5,
    Bgra32 = 6,
    Abgr32 = 7,
    Gray8 = 8,
    Doom256Colors8 = 9,
}

impl ScreenFormat {
    /// Maps a raw C ABI integer back to a [`ScreenFormat`].
    pub fn from_raw(value: i32) -> Option<Self> {
        Some(match value {
            0 => ScreenFormat::Crcgcb,
            1 => ScreenFormat::Rgb24,
            2 => ScreenFormat::Rgba32,
            3 => ScreenFormat::Argb32,
            4 => ScreenFormat::Cbcgcr,
            5 => ScreenFormat::Bgr24,
            6 => ScreenFormat::Bgra32,
            7 => ScreenFormat::Abgr32,
            8 => ScreenFormat::Gray8,
            9 => ScreenFormat::Doom256Colors8,
            _ => return None,
        })
    }
}

/// Screen resolution. Mirrors `vizdoom::ScreenResolution` (sequential from 0).
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenResolution {
    Res160x120 = 0,
    Res200x125,
    Res200x150,
    Res256x144,
    Res256x160,
    Res256x192,
    Res320x180,
    Res320x200,
    Res320x240,
    Res320x256,
    Res400x225,
    Res400x250,
    Res400x300,
    Res512x288,
    Res512x320,
    Res512x384,
    Res640x360,
    Res640x400,
    Res640x480,
    Res800x450,
    Res800x500,
    Res800x600,
    Res1024x576,
    Res1024x640,
    Res1024x768,
    Res1280x720,
    Res1280x800,
    Res1280x960,
    Res1280x1024,
    Res1400x787,
    Res1400x875,
    Res1400x1050,
    Res1600x900,
    Res1600x1000,
    Res1600x1200,
    Res1920x1080,
}

/// Automap rendering mode. Mirrors `vizdoom::AutomapMode`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutomapMode {
    Normal = 0,
    Whole,
    Objects,
    ObjectsWithSize,
}

/// Audio sampling rate. Mirrors `vizdoom::SamplingRate`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SamplingRate {
    Sr11025 = 0,
    Sr22050,
    Sr44100,
}

/// Input buttons. Mirrors `vizdoom::Button`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Attack = 0,
    Use = 1,
    Jump = 2,
    Crouch = 3,
    Turn180 = 4,
    AltAttack = 5,
    Reload = 6,
    Zoom = 7,
    Speed = 8,
    Strafe = 9,
    MoveRight = 10,
    MoveLeft = 11,
    MoveBackward = 12,
    MoveForward = 13,
    TurnRight = 14,
    TurnLeft = 15,
    LookUp = 16,
    LookDown = 17,
    MoveUp = 18,
    MoveDown = 19,
    Land = 20,
    SelectWeapon1 = 21,
    SelectWeapon2 = 22,
    SelectWeapon3 = 23,
    SelectWeapon4 = 24,
    SelectWeapon5 = 25,
    SelectWeapon6 = 26,
    SelectWeapon7 = 27,
    SelectWeapon8 = 28,
    SelectWeapon9 = 29,
    SelectWeapon0 = 30,
    SelectNextWeapon = 31,
    SelectPrevWeapon = 32,
    DropSelectedWeapon = 33,
    ActivateSelectedItem = 34,
    SelectNextItem = 35,
    SelectPrevItem = 36,
    DropSelectedItem = 37,
    LookUpDownDelta = 38,
    TurnLeftRightDelta = 39,
    MoveForwardBackwardDelta = 40,
    MoveLeftRightDelta = 41,
    MoveUpDownDelta = 42,
}

/// Game variables exposed in state. Mirrors `vizdoom::GameVariable`
/// (sequential from 0; `USER0` is reserved and intentionally omitted).
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameVariable {
    KillCount = 0,
    ItemCount,
    SecretCount,
    FragCount,
    DeathCount,
    HitCount,
    HitsTaken,
    DamageCount,
    DamageTaken,
    Health,
    Armor,
    Dead,
    OnGround,
    AttackReady,
    AltAttackReady,
    SelectedWeapon,
    SelectedWeaponAmmo,
    Ammo0,
    Ammo1,
    Ammo2,
    Ammo3,
    Ammo4,
    Ammo5,
    Ammo6,
    Ammo7,
    Ammo8,
    Ammo9,
    Weapon0,
    Weapon1,
    Weapon2,
    Weapon3,
    Weapon4,
    Weapon5,
    Weapon6,
    Weapon7,
    Weapon8,
    Weapon9,
    PositionX,
    PositionY,
    PositionZ,
    Angle,
    Pitch,
    Roll,
    ViewHeight,
    VelocityX,
    VelocityY,
    VelocityZ,
    CameraPositionX,
    CameraPositionY,
    CameraPositionZ,
    CameraAngle,
    CameraPitch,
    CameraRoll,
    CameraFov,
    PlayerNumber,
    PlayerCount,
    Player1FragCount,
    Player2FragCount,
    Player3FragCount,
    Player4FragCount,
    Player5FragCount,
    Player6FragCount,
    Player7FragCount,
    Player8FragCount,
    Player9FragCount,
    Player10FragCount,
    Player11FragCount,
    Player12FragCount,
    Player13FragCount,
    Player14FragCount,
    Player15FragCount,
    Player16FragCount,
    User1,
    User2,
    User3,
    User4,
    User5,
    User6,
    User7,
    User8,
    User9,
    User10,
    User11,
    User12,
    User13,
    User14,
    User15,
    User16,
    User17,
    User18,
    User19,
    User20,
    User21,
    User22,
    User23,
    User24,
    User25,
    User26,
    User27,
    User28,
    User29,
    User30,
    User31,
    User32,
    User33,
    User34,
    User35,
    User36,
    User37,
    User38,
    User39,
    User40,
    User41,
    User42,
    User43,
    User44,
    User45,
    User46,
    User47,
    User48,
    User49,
    User50,
    User51,
    User52,
    User53,
    User54,
    User55,
    User56,
    User57,
    User58,
    User59,
    User60,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_values_match_cpp() {
        assert_eq!(Mode::Player as i32, 0);
        assert_eq!(Mode::AsyncSpectator as i32, 3);
    }

    #[test]
    fn screen_format_values_match_cpp() {
        assert_eq!(ScreenFormat::Crcgcb as i32, 0);
        assert_eq!(ScreenFormat::Doom256Colors8 as i32, 9);
        assert_eq!(ScreenFormat::from_raw(1), Some(ScreenFormat::Rgb24));
        assert_eq!(ScreenFormat::from_raw(99), None);
    }

    #[test]
    fn screen_resolution_values_match_cpp() {
        assert_eq!(ScreenResolution::Res160x120 as i32, 0);
        assert_eq!(ScreenResolution::Res640x480 as i32, 18);
        assert_eq!(ScreenResolution::Res1920x1080 as i32, 35);
    }

    #[test]
    fn button_values_match_cpp() {
        assert_eq!(Button::Attack as i32, 0);
        assert_eq!(Button::MoveLeft as i32, 11);
        assert_eq!(Button::MoveRight as i32, 10);
        assert_eq!(Button::MoveUpDownDelta as i32, 42);
    }

    #[test]
    fn game_variable_values_match_cpp() {
        assert_eq!(GameVariable::KillCount as i32, 0);
        assert_eq!(GameVariable::Health as i32, 9);
        assert_eq!(GameVariable::Ammo2 as i32, 19);
        assert_eq!(GameVariable::PositionX as i32, 37);
        assert_eq!(GameVariable::Player16FragCount as i32, 71);
        assert_eq!(GameVariable::User1 as i32, 72);
        assert_eq!(GameVariable::User60 as i32, 131);
    }

    #[test]
    fn automap_and_sampling_values_match_cpp() {
        assert_eq!(AutomapMode::ObjectsWithSize as i32, 3);
        assert_eq!(SamplingRate::Sr44100 as i32, 2);
    }
}
