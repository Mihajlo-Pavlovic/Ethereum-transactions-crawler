use chrono::prelude::*;
use common::models as c_models;
use stylist::Style;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod service;

const STYLE_FILE: &str = include_str!("main.css");

pub enum Msg {
    GetData,
    ProcessData(Option<c_models::AccountData>),
}

struct App {
    error_msg: String,
    input_ref: NodeRef,
    input_from: NodeRef,
    input_to: NodeRef,
    received_data: Option<c_models::AccountData>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
            input_from: NodeRef::default(),
            input_to: NodeRef::default(),
            received_data: None,
            error_msg: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetData => {
                log::debug!("Get data");
                let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
                let from_input = self.input_from.cast::<HtmlInputElement>().unwrap();
                let to_input = self.input_to.cast::<HtmlInputElement>().unwrap();
                let address = if input.value().trim().is_empty() {
                    "".to_string()
                } else {
                    input.value().trim().to_string()
                };
                let from: i32 = match from_input.value().parse() {
                    Ok(from) => from,
                    Err(_err) => {
                        // log::error!("error ocurred parsing value: {}", err);
                        0
                    }
                };
                let to: i32 = match to_input.value().parse() {
                    Ok(to) => to,
                    Err(_err) => {
                        // log::error!("error ocurred parsing value: {}", err);
                        99999999
                    }
                };
                input.set_value("");
                let params = c_models::QueryParams {
                    address: address,
                    from: from,
                    to: to,
                    page: 1,
                    offset: 1000,
                    sort: "asc".to_string(),
                };
                if from > to || from < 0 || to < 0 {
                    self.error_msg = "Invalid block numbers!".to_string();
                } else if params.address == "".to_string() {
                    self.error_msg = "Invalid wallet address!".to_string();
                } else {
                    self.error_msg = "".to_string();
                    ctx.link().send_future(async move {
                        let result = service::get_account_data(&params).await;
                        Msg::ProcessData(result)
                    });
                }
            }
            Msg::ProcessData(data) => {
                log::debug!("Received data: {:?}", data);
                self.received_data = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_click = link.callback(|_e: MouseEvent| {
            log::debug!("Clicked");
            Msg::GetData
        });

        html! {
            <div>
            <h1>{"Ethereum transactions crawler frontend"}</h1>
            <div>
                    {"Address:"}
                    <input style="width:350px;" type="text" ref={self.input_ref.clone()} />
                    <br/>
                    {"From block (if left empty it will be 0):"}
                    <input style="width:100px;" type="number" ref={self.input_from.clone()} min="0" />
                    <br/>
                    {"To block (if left empty it will be 99999999):"}
                    <input style="width:100px;" type="number" ref={self.input_to.clone()} min="0"/>
                    <br/>
                    <button onclick={on_click}>{"Get Account Data"}</button>
                    <br/>
                    <lable style="color:red">{self.error_msg.clone()}</lable>
                </div>
                <div>
                { match self.received_data.clone() {
                    Some(val) => {
                        show_account(&val)
                    },
                    _ => html! {
                        <div>
                            {"Please enter an address"}
                        </div>
                    }
                }}
                </div>
            </div>
        }
    }
}

fn show_account(account: &c_models::AccountData) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
    let normal_tx = account.normal_transactions.clone();
    let tx_html = normal_tx.into_iter().map(|tx| {
        html! {
            <tr key={tx.hash}>
                <td>{time(tx.time_stamp)}</td>
                <td>{tx.block_number}</td>
                <td>{tx.from}</td>
                <td>{tx.to}</td>
                <td>{wei_to_eth(tx.value)}</td>
                <td>{txn_fee(tx.gas_price ,tx.cumulative_gas_used)}</td>
                // <td>{tx.contract_address}</td>
                // <td>{tx.function_name}</td>
            </tr>
        }
    });

    html! {
        <div class={stylesheet}>
            <div style="text-align:center">
            { format!("Address: {}, Balance: {}, Normal tx: {}",
            account.address, wei_to_eth(account.balance.to_string()), account.normal_transactions.len()
            ) }
            </div>
            <div class={classes!("table-wrapper")}>
                <table class={classes!("fl-table")}>
                    <thead>
                        <tr>
                        <th>{"Age"}</th>
                        <th>{"Block"}</th>
                        <th>{"From"}</th>
                        <th>{"To"}</th>
                        <th>{"Value"}</th>
                        <th>{"Gas Price"}</th>
                        // <th>{"Contract"}</th>
                        // <th>{"Function"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for tx_html}
                    </tbody>
                </table>
            </div>
        </div>

    }
}

// fn wei_to_eth(wei_val: String) -> f64 {
//     let res = wei_val.parse::<u64>().unwrap() as f64;
//     let res = res / 1_000_000_000_000_000_000.0;
//     res
// }

fn txn_fee(price: String, used: String) -> String {
    let p = price.parse::<u128>().unwrap() as f64;
    let u = used.parse::<u128>().unwrap() as f64;
    let res = p * u;
    return wei_to_eth(res.to_string());
}
fn wei_to_eth(wei_val: String) -> String {
    let res = wei_val.parse::<u128>().unwrap() as f64;
    let res = res / 1_000_000_000_000_000_000.0;
    return res.to_string();
}
fn time(ts: String) -> String {
    let nt = NaiveDateTime::from_timestamp(ts.parse::<i64>().unwrap(), 0);
    let dt: DateTime<Utc> = DateTime::from_utc(nt, Utc);
    let res = dt.format("%Y-%m-%d %H:%M:%S");
    return res.to_string();
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
