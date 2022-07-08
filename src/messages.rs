use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessagesProps {

}

#[function_component(Messages)]
pub fn messages_component(MessagesProps: &MessagesProps) -> Html{
    html!{
        <div class={"messages"}>
            <form>
                <p>{"Messages will go here!"}</p> <br/>
                <input type="text" id="message-input" name="message-input"  /> <br/>
                <input type="submit" id="submit"/>
            </ form>    
        </div>
    }
}