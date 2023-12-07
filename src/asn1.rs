mod asn1_viewer;
mod hex_view;
mod scheme;

use std::rc::Rc;

use asn1_parser::Asn1;
use web_sys::KeyboardEvent;
use yew::{classes, function_component, html, use_reducer, use_state, Callback, Html, Reducible};

use crate::asn1::asn1_viewer::Asn1Viewer;
use crate::asn1::hex_view::HexViewer;
use crate::common::{ByteInput, Checkbox};

pub const TEST_ASN1: &[u8] = &[
    // 48, 50, 161, 17, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 162, 9, 12, 7,
    // 113, 107, 97, 116, 105, 111, 110, 163, 18, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156, 221, 80, 86,
    // 181, 110,
    // 48, 44, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 12, 7, 113, 107, 97, 116,
    // 105, 111, 110, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156, 221, 80, 86, 181, 110,
    48, 130, 2, 30, 4, 7, 58, 232, 40, 24, 17, 216, 176, 4, 11, 216, 44, 74, 26, 137, 109, 11, 173, 211, 185, 135, 48,
    129, 255, 3, 5, 7, 67, 253, 55, 182, 12, 48, 125, 242, 181, 151, 128, 241, 174, 168, 128, 42, 243, 146, 170, 138,
    47, 10, 123, 242, 187, 129, 138, 241, 183, 182, 166, 195, 168, 240, 146, 188, 162, 226, 128, 174, 104, 61, 100,
    103, 242, 147, 175, 190, 36, 61, 103, 96, 58, 36, 1, 1, 0, 12, 48, 127, 241, 148, 178, 169, 61, 243, 128, 178, 143,
    195, 141, 0, 0, 59, 58, 51, 58, 242, 154, 140, 138, 243, 136, 172, 139, 96, 58, 46, 240, 159, 149, 180, 119, 97,
    39, 226, 128, 174, 241, 185, 169, 191, 241, 178, 167, 141, 36, 4, 5, 50, 178, 83, 20, 77, 4, 1, 140, 12, 58, 242,
    167, 134, 180, 61, 239, 187, 191, 195, 181, 242, 159, 149, 180, 47, 96, 242, 140, 178, 173, 226, 128, 174, 240,
    159, 149, 180, 242, 189, 135, 177, 88, 194, 165, 0, 58, 114, 209, 168, 241, 172, 148, 154, 240, 159, 149, 180, 41,
    37, 242, 190, 156, 163, 235, 135, 172, 194, 165, 1, 1, 0, 4, 12, 117, 86, 112, 180, 20, 202, 224, 28, 58, 3, 133,
    90, 4, 15, 212, 214, 53, 154, 145, 2, 117, 175, 243, 103, 181, 102, 19, 251, 188, 3, 20, 5, 221, 77, 67, 230, 172,
    240, 96, 163, 227, 181, 175, 194, 248, 31, 235, 105, 46, 230, 38, 3, 17, 5, 155, 82, 70, 117, 135, 62, 42, 165,
    241, 155, 147, 173, 209, 54, 160, 138, 4, 4, 129, 51, 94, 101, 48, 46, 12, 44, 231, 158, 163, 58, 50, 63, 96, 10,
    53, 111, 51, 123, 63, 240, 184, 179, 161, 209, 168, 123, 242, 150, 186, 157, 243, 170, 137, 140, 200, 186, 242,
    155, 156, 156, 92, 0, 27, 195, 149, 92, 242, 156, 178, 152, 3, 18, 0, 206, 190, 97, 201, 75, 125, 225, 116, 109,
    226, 236, 4, 19, 9, 7, 185, 100, 12, 57, 243, 176, 171, 184, 58, 92, 243, 166, 183, 187, 243, 132, 154, 159, 243,
    165, 189, 155, 61, 36, 46, 66, 239, 191, 189, 240, 177, 152, 141, 241, 168, 156, 143, 13, 194, 134, 39, 240, 172,
    142, 137, 46, 240, 159, 149, 180, 10, 243, 133, 180, 182, 69, 242, 128, 164, 156, 122, 1, 1, 0, 3, 16, 1, 66, 115,
    229, 233, 85, 68, 237, 69, 93, 254, 218, 104, 75, 133, 241, 4, 18, 75, 232, 138, 24, 247, 158, 233, 154, 181, 156,
    155, 252, 71, 105, 43, 215, 211, 160, 3, 23, 2, 102, 29, 9, 2, 92, 205, 26, 162, 54, 221, 33, 80, 194, 82, 99, 110,
    161, 116, 102, 123, 221, 240, 12, 61, 241, 164, 185, 138, 243, 177, 154, 148, 243, 150, 162, 184, 209, 168, 123,
    46, 240, 147, 130, 133, 63, 242, 147, 170, 174, 226, 128, 174, 11, 241, 174, 152, 137, 27, 37, 10, 241, 164, 144,
    156, 243, 148, 132, 139, 241, 150, 160, 154, 241, 169, 185, 175, 226, 128, 174, 226, 128, 174, 0, 70, 45,
];

pub enum HighlightAction {
    Show(u64),
    Hide(u64),
}

#[derive(Debug, Default, Clone)]
pub struct Highlight {
    nodes: Vec<u64>,
}

impl Highlight {
    fn show(&mut self, id: u64) {
        self.hide(id);
        self.nodes.push(id);
    }

    fn hide(&mut self, id: u64) {
        while let Some(index) = self.nodes.iter().position(|asn1_id| *asn1_id == id) {
            self.nodes.remove(index);
        }
    }

    fn current(&self) -> Option<u64> {
        self.nodes.last().copied()
    }
}

impl Reducible for Highlight {
    type Action = HighlightAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut highlight = self.as_ref().clone();
        match action {
            HighlightAction::Show(id) => highlight.show(id),
            HighlightAction::Hide(id) => highlight.hide(id),
        }
        Rc::new(highlight)
    }
}

#[function_component(Asn1ParserPage)]
pub fn asn1_parser_page() -> Html {
    let auto_decode = use_state(|| true);
    let raw_asn1 = use_state(|| TEST_ASN1.to_vec());
    let parsed_asn1 = use_state(Asn1::default);

    let set_auto_decode = auto_decode.setter();
    let set_checked = Callback::from(move |checked| {
        set_auto_decode.set(checked);
    });

    let parse_asn1 = Callback::from(move |_: ()| {
        //
    });
    let go = parse_asn1.clone();
    let onclick = Callback::from(move |_| {
        parse_asn1.emit(());
    });

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            go.emit(());
        }
    });

    let raw_asn1_setter = raw_asn1.setter();

    let ctx = use_reducer(Highlight::default);
    let asn1_dispatcher = ctx.dispatcher();
    let hex_dispatcher = ctx.dispatcher();

    html! {
        <div class={classes!("vertical", "asn1-page")} {onkeydown}>
            <ByteInput bytes={(*raw_asn1).clone()} setter={Callback::from(move |data| raw_asn1_setter.set(data))} placeholder={"asn1 data".to_owned()} rows={10} />
            <div class="horizontal">
                <button class="action-button" {onclick}>{"Process"}</button>
                <Checkbox id={"auto-decode-asn1".to_owned()} name={"auto-decode".to_owned()} checked={*auto_decode} {set_checked} />
            </div>
            <div class="asn1-viewers">
                <Asn1Viewer
                    structure={(*parsed_asn1).clone()}
                    cur_node={(*ctx).current()}
                    set_cur_node={move |action| asn1_dispatcher.dispatch(action)}
                />
                <HexViewer
                    raw_data={(*raw_asn1).clone()}
                    structure={(*parsed_asn1).clone()}
                    cur_node={(*ctx).current()}
                    set_cur_node={move |action| hex_dispatcher.dispatch(action)}
                />
            </div>
        </div>
    }
}
