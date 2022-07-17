use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessagesProps {

}

#[function_component(Messages)]
pub fn messages_component(props: &MessagesProps) -> Html{
    let last_input = use_state(|| String::from(" "));
    let oninput  ={ 
        let last_input = last_input.clone();
        Callback::from(move |e:InputEvent| {
        let input: String = e.data().unwrap();
        last_input.set(input);
    })};

    html! {
        <div class={"messages"}>
            <form>
                <p>{"Messages will go here!"}</p> <br/>
                <input type="text" id="message-input" name="message-input" {oninput} /> <br/>
                <input type="submit" id="submit"/>
            </ form>  
            <p>{format!("{:?}", *last_input)}</p> 
        </div>
    }
}