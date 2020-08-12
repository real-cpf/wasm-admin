use yew::{html, Component,ChangeData, ComponentLink, Html, MouseEvent,Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use std::str::FromStr;

pub struct DBForm{
    link:ComponentLink<Self>,
    dataset:DbStruct,
    now_table:usize,
}
#[derive(Clone,Default)]
pub struct DbStruct{
    pub num_tables:usize,
    pub tables:Vec<DataTable>,
}
#[derive(Clone,Default)]
pub struct DataTable{
    pub col:Vec<String>,
    pub data:Vec<String>,
}

#[derive(Clone)]
pub enum Msg{
    Ignore,
    SelectTable(usize),
}





impl Component for DBForm{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        let test_col=vec![String::from("A"),String::from("B"),String::from("C"),String::from("D"),String::from("E"),String::from("F")];
        let test_row1=vec![String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),
        String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6")];
        let test_table=DataTable{col:test_col,data:test_row1};
        let test_dataset=DbStruct{
            num_tables:1,
            tables:vec![test_table],
        };

        DBForm{
            link,
            dataset:test_dataset,
            now_table:0,
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        match msg{
            Msg::Ignore=>{

            },
            Msg::SelectTable(index)=>{
                self.now_table=index;
            },
        }
        true
    }  
    fn change(&mut self, _props: Self::Properties)->ShouldRender{

        true
    }

    fn view(&self)->Html{
        let select_change=&self.link.callback(move |value|{
            let mut index:usize=0;
            if let ChangeData::Select(ss) =value{
                index=usize::from_str(ss.value().as_str()).unwrap();
            }
            Msg::SelectTable(index)
            // Msg::Ignore
        });
        html!{
            <div>

            <select name="now-table" onchange=select_change id="now-table">
            <option value=0>{"第一个"}</option>
            <option value=1>{"第二个"}</option>
            <option value=2>{"第三个"}</option>
            </select>
            <div>{self.now_table}</div>
            // <textarea name="script-box" id="script-box" cols="30" rows="10">
            // {"abc"}
            // </textarea>

            <div>
            {self.render_table(self.now_table)}
            </div>

            </div>
        }
    }



}

impl DBForm{
    fn render_table(&self,index:usize)->VNode{
    if index>self.dataset.num_tables-1{
        html!{
            <div>{"no this talbe"}</div>
        }
    }else{
        let dt=self.dataset.tables[index].clone();
        let col=dt.col.clone();
        let data=dt.data.clone();
        html!{
            <table class="table-style">
            <thead>       
                {
                    for col.iter().map(|name| {html! { <td>{name}</td> }})
                }
            </thead>
            
            {
                for data.iter().map(|row|{
                    let rs:Vec<&str> =row.split("<>").collect();
                    html!{
                        <tr>
                        {
                            for rs.iter().map(|e|{
                                html!{
                                    <td>{e}</td>
                                }
                            })
                        }
                        </tr>
                    }
                })
            }
    
            </table>
        }
    }

    }
}

