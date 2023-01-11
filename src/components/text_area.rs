use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps {
    pub data_test_id: String,
    pub value: Option<String>,
    pub onchange: Callback<String>,
    pub id: String,
}

#[function_component(TextArea)]
pub fn text_input(props: &TextAreaProps) -> Html {
    let value = use_state(String::new);
    let has_loaded = use_state(|| false);

    let oninput = {
        let props_onchange = props.onchange.clone();
        let value = value.clone();

        Callback::from(move |event: InputEvent| {
            let input_value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            props_onchange.emit(input_value.clone());
            value.set(input_value);
        })
    };

    {
        let value = value.clone();
        let has_loaded = has_loaded;
        let props_value = props.value.clone();
        use_effect(move || {
            if let Some(props_value) = props_value {
                if !*has_loaded && value.is_empty() {
                    value.set(props_value);
                    has_loaded.set(true);
                }
            }
            || {}
        })
    }

    html! {
      <>
        <textarea
          class="code"
          spellcheck="false"
          data-testid={props.data_test_id.clone()}
          id={props.id.clone()}
          value={value.deref().clone()}
          {oninput}
        />
      </>
    }
}
