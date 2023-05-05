use yew::{AttrValue, Callback, function_component, Html, html, use_state};
use super::Square::Square;
#[function_component]
pub fn Board() -> Html {
    let boardState = use_state(|| vec![AttrValue::default();9]);

    let handlerClick = {
        |index:usize| {
            let boardState = boardState.clone();
            Callback::from(move|_| {
                let mut bs = (*boardState).clone();
                bs[index] = "X".into();
                boardState.set(bs);
            })
        }
    };


    html!{
       <>
        <div class="board-row">
            <Square value={(*boardState)[0].clone()} onSquareClick={handlerClick(0)}/>
            <Square value={(*boardState)[1].clone()} onSquareClick={handlerClick(1)}/>
            <Square value={(*boardState)[2].clone()} onSquareClick={handlerClick(2)}/>
        </div>

    <div class="board-row">
            <Square value={(*boardState)[3].clone()} onSquareClick={handlerClick(3)}/>
            <Square value={(*boardState)[4].clone()} onSquareClick={handlerClick(4)}/>
            <Square value={(*boardState)[5].clone()} onSquareClick={handlerClick(5)}/>
        </div>

        <div class="board-row">
            <Square value={(*boardState)[6].clone()} onSquareClick={handlerClick(6)}/>
            <Square value={(*boardState)[7].clone()} onSquareClick={handlerClick(7)}/>
            <Square value={(*boardState)[8].clone()} onSquareClick={handlerClick(8)}/>
        </div>


        </>
    }
}