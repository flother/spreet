use resvg::usvg::Rect;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};

/// Custom Serde field serialiser for [`Rect`].
///
/// Serialises an [`f32`] with a zero fractional part as a [`u32`], and otherwise as an `f32`
/// unchanged. Allows JSON outputted by Spreet to match the JavaScript style of intermingling
/// integers and floats (because there's only one number type in JavaScript).
///
/// Used to serialise a stretchable icon's [content area](`super::Sprite::content_area`). The
/// serialised data is almost always integers (exceptions being transformed or rotated elements) and
/// so it's a waste to serialise the extra `.0` every time.
pub fn serialize_rect<S>(rect: &Option<Rect>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match rect {
        Some(r) => {
            let mut seq = serializer.serialize_seq(Some(4))?;
            for num in [r.left(), r.top(), r.right(), r.bottom()] {
                if num.fract() == 0.0 {
                    seq.serialize_element(&(num.round() as i32))?;
                } else {
                    seq.serialize_element(&((num * 1e3).round() / 1e3))?;
                }
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Custom Serde field serializer for a vector of [`Rect`]s.
///
/// Serialises the left and right edges of each `Rect` as a [`u32`] if the value has no fractional
/// part, or an unchanged [`f32`] otherwise. Allows JSON outputted by Spreet to match the JavaScript
/// style of intermingling integers and floats (because there's only one number type in JavaScript).
///
/// Used to serialise a stretchable icon's [stretch-x areas](`super::Sprite::stretch_x_areas`). The
/// serialised data is almost always integers (exceptions being transformed or rotated elements) and
/// so it's a waste to serialise the extra `.0` every time.
pub fn serialize_stretch_x_area<S>(
    rects: &Option<Vec<Rect>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match rects {
        Some(rects) => {
            let mut seq = serializer.serialize_seq(Some(rects.len()))?;
            for rect in rects {
                let line = [icon_number(rect.left()), icon_number(rect.right())];
                seq.serialize_element(&line)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Custom Serde field serializer for a vector of [`Rect`]s.
///
/// Serialises the top and bottom edges of each `Rect` as a [`u32`] if the value has no fractional
/// part, or an unchanged [`f32`] otherwise. Allows JSON outputted by Spreet to match the JavaScript
/// style of intermingling integers and floats (because there's only one number type in JavaScript).
///
/// Used to serialise a stretchable icon's [stretch-y areas](`super::Sprite::stretch_y_areas`). The
/// serialised data is almost always integers (exceptions being transformed or rotated elements) and
/// so it's a waste to serialise the extra `.0` every time.
pub fn serialize_stretch_y_area<S>(
    rects: &Option<Vec<Rect>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match rects {
        Some(rects) => {
            let mut seq = serializer.serialize_seq(Some(rects.len()))?;
            for rect in rects {
                let line = [icon_number(rect.top()), icon_number(rect.bottom())];
                seq.serialize_element(&line)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Represents a number, whether integer or floating point, that can be serialised to JSON.
#[derive(Serialize)]
#[serde(untagged)]
enum Number {
    Int(u32),
    Float(f32),
}

/// Converts an [`f32`] to a [`Number`], so it can be serialised to JSON in its most minimal form.
fn icon_number(num: f32) -> Number {
    if num.fract() == 0.0 {
        Number::Int(num.round() as u32)
    } else {
        Number::Float((num * 1e3).round() / 1e3)
    }
}
