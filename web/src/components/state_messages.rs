use yewdux::prelude::*;
use crate::components::context::UIState;
use std::rc::Rc;

pub enum UIStateMsg {
    ClearErrorMessage,
    ClearInfoMessage,
}

impl Reducer<UIState> for UIStateMsg {
    fn apply(self, mut state: Rc<UIState>) -> Rc<UIState> {
        let state = Rc::make_mut(&mut state);

        match self {
            UIStateMsg::ClearErrorMessage => {
                state.error_message = None;
            },
            UIStateMsg::ClearInfoMessage => {
                state.info_message = None;
            },
        }

        (*state).clone().into()
    }
}
