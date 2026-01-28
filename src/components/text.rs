use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element Text variants - themed wrappers around Makepad Label/H1-H4
    // RNE: h1=40, h2=34, h3=28, h4=22
    pub ElementH1 = <H1> {
        draw_text: {
            color: #212121,
            text_style: { font_size: 40.0 }
        }
    }

    pub ElementH2 = <H2> {
        draw_text: {
            color: #212121,
            text_style: { font_size: 34.0 }
        }
    }

    pub ElementH3 = <H3> {
        draw_text: {
            color: #333333,
            text_style: { font_size: 28.0 }
        }
    }

    pub ElementH4 = <H4> {
        draw_text: {
            color: #333333,
            text_style: { font_size: 22.0 }
        }
    }

    pub ElementBody = <P> {
        draw_text: {
            color: #424242,
            text_style: { font_size: 14.0 }
        }
    }

    pub ElementCaption = <Label> {
        draw_text: {
            color: #757575,
            text_style: { font_size: 12.0 }
        }
    }

    pub ElementOverline = <Label> {
        draw_text: {
            color: #9E9E9E,
            text_style: { font_size: 10.0 }
        }
    }
}
