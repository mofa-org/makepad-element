// Component tests for makepad-element
// Tests will be added as components are implemented

#[test]
fn theme_colors_valid() {
    use makepad_element::theme::colors::*;
    // Verify primary colors are in valid range
    assert!(ELEMENT_PRIMARY.x >= 0.0 && ELEMENT_PRIMARY.x <= 1.0);
    assert!(ELEMENT_PRIMARY.y >= 0.0 && ELEMENT_PRIMARY.y <= 1.0);
    assert!(ELEMENT_PRIMARY.z >= 0.0 && ELEMENT_PRIMARY.z <= 1.0);
    assert!(ELEMENT_PRIMARY.w >= 0.0 && ELEMENT_PRIMARY.w <= 1.0);
}

#[test]
fn theme_spacing_positive() {
    use makepad_element::theme::spacing::*;
    assert!(ELEMENT_SPACING_XS > 0.0);
    assert!(ELEMENT_SPACING_SM > ELEMENT_SPACING_XS);
    assert!(ELEMENT_SPACING_MD > ELEMENT_SPACING_SM);
    assert!(ELEMENT_SPACING_LG > ELEMENT_SPACING_MD);
    assert!(ELEMENT_SPACING_XL > ELEMENT_SPACING_LG);
}

#[test]
fn theme_typography_sizes() {
    use makepad_element::theme::typography::*;
    assert!(ELEMENT_FONT_H1 > ELEMENT_FONT_H2);
    assert!(ELEMENT_FONT_H2 > ELEMENT_FONT_H3);
    assert!(ELEMENT_FONT_H3 > ELEMENT_FONT_H4);
    assert!(ELEMENT_FONT_H4 > ELEMENT_FONT_BODY);
    assert!(ELEMENT_FONT_BODY > ELEMENT_FONT_CAPTION);
}
