use yew::{html, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;

pub struct DBForm{
    link:ComponentLink<Self>,
    dataset:DbStruct,
}
#[derive(Clone)]
pub struct DbStruct{
    col:Vec<String>,
    data:Vec<Vec<String>>,
}

#[derive(Clone)]
pub enum Msg{
    Ignore,
}

impl Component for DBForm{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        let col=vec![String::from("col1"),String::from("col2"),String::from("col3")];
        let data=vec![
            vec![String::from("v1,"),String::from("v2,"),String::from("v33")],
            vec![String::from("v1,"),String::from("v2,"),String::from("v33")],
            vec![String::from("v1,"),String::from("v2,"),String::from("v33")],
        ];
        let dataset=DbStruct{
            col:col,
            data:data,
        };

        DBForm{
            link,
            dataset:dataset,
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        true
    }  
    fn change(&mut self, _props: Self::Properties)->ShouldRender{
        true
    }

    fn view(&self)->Html{
        html!{
            <div>
            // <textarea name="script-box" id="script-box" cols="30" rows="10">
            // {"abc"}
            // </textarea>
            <table>
                <thead>
                {
                    for self.dataset.col.iter().map(|v|{
                        html!{
                            <td>{v}</td>
                        }
                    })
                }
                   
                </thead>
                
                {
                    for self.dataset.data.iter().map(move |v|{
                        let now_v=v.clone();
                        html!{
                            <tr>
                            <td>{now_v}</td>
                            </tr>
                        }
                    })
                }
                    
               
            </table>

            </div>
        }
    }



}