use yew::{html, Component,ChangeData, ComponentLink, Html, MouseEvent,Properties, ShouldRender};
use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use std::str::FromStr;

pub struct DBForm{
    link:ComponentLink<Self>,
    dataset:DbStruct,
    now_table:usize,
    script_value:String,
    show_log:String,
}
#[derive(Clone,Default)]
pub struct DbStruct{
    pub name_kv:Vec<(String,usize)>,
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
    InsertTable(DataTable),
    InsertRow,
    UpdateScriptValue(String),
}





impl Component for DBForm{
    type Message=Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>)->Self{
        let test_col=vec![String::from("A"),String::from("B"),String::from("C"),String::from("D"),String::from("E"),String::from("F")];
        let test_row1=vec![String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),
        String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6"),String::from("1<>2<>3<>4<>5<>6")];
        let test_table=DataTable{ col:test_col,data:test_row1};
        let test_dataset=DbStruct{
            name_kv:vec![(String::from("test_table"),0_usize)],
            num_tables:1,
            tables:vec![test_table],
        };

        DBForm{
            link,
            dataset:test_dataset,
            now_table:0,
            script_value:String::from(""),
            show_log:String::from("ok"),
        }
    }

    fn update(&mut self,msg: Self::Message)->ShouldRender{
        match msg{
            Msg::Ignore=>{

            },
            Msg::SelectTable(index)=>{
                self.now_table=index;
            },
            Msg::InsertRow=>{
                let row=self.script_value.clone();
                let index=self.now_table;
                let col_len=self.dataset.tables[index].col.len();
                let rs:Vec<&str> =row.split(",").collect();
                if rs.len()==col_len{
                    let _row=row.replace(",", "<>");
                    self.dataset.tables[index].data.push(_row);
                    self.script_value=String::from("");
                    self.show_log=String::from("新增成功！！！")
                }else{
                    self.show_log=format!("col_len:{},rs_len:{}列的数量不正确！！！",col_len,rs.len())
                }

            },
            Msg::InsertTable(table)=>{

            },Msg::UpdateScriptValue(script)=>{
                self.script_value=script;
            }
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
        });
        let script_value_change=&self.link.callback(|ev: InputData|{
            Msg::UpdateScriptValue(ev.value)
        });
        let insert_row_value=&self.link.callback(|_| {Msg::InsertRow });
        html!{
            <div>

            <div class="log-style">
            {
                format!("执行日志：{}",&self.show_log)
            }
            </div>

            <div>
            <input oninput=script_value_change type="text" name="inputs"  id="inputs" value=&self.script_value  />
            <button onclick=insert_row_value >{"新增"}</button>
            </div>
            
            <div><span>{"当前选择表："}</span>
            <select name="now-table" class="select-style" onchange=select_change id="now-table">
            // <option value=0>{"第一个"}</option>
            // <option value=1>{"第二个"}</option>
            // <option value=2>{"第三个"}</option>
            {
                for self.dataset.name_kv.iter().map(|v|{
                    html!{
                        <option value=v.1>{v.0.clone()}</option>
                    }
                })
            }
            </select>
            </div>
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

