use tokio;
use super::types::*;
use proc_qq::re_exports::ricq::client::event::GroupMessageEvent;
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait,
    Module,
};


pub static mut sender : Option<tokio::sync::mpsc::Sender<(PoweredPulse ,PoweredPulse)>> = None;


#[event]
async fn bot(event: &GroupMessageEvent) -> anyhow::Result<bool> {

  let content = event.message_content();
  
  let _sender = unsafe {
    match &sender
    { None => return Ok(false)
    , Some(x) => x }
  }; 

  if content.contains("#dg") {
    
    if content.contains("help") {
      event.send_message_to_source(
        concat!
        ( "请按格式发送 \" #dg 强度 频率 \" 或者 \" #dg 强度 频率 脉冲宽度 \".\n\n"
        , "其中 \n  强度 ∈ [0, 290] ∩ ℕ,\n 频率 ∈ [10, 1000] ∩ ℕ,\n 脉冲宽度 ∈ [0, 31] ∩ ℕ.\n\n"
        , "P.S. 当脉冲宽度大于 20 时更容易引起刺痛.").parse_message_chain()
      ).await?;
      return Ok(true);
    }
    
    let pulse = content.parse::<PoweredPulse>();
    match pulse {

      Ok(p) => { 

        match _sender.send((p.clone(), p)).await {
          Err(e) => {
            println!("ERR!!! {}", e);
          },
          _ => {},
        };
        println!("Bot: msg sended, {}", content.parse::<PoweredPulse>().unwrap());
      },

      Err(e) => { 
        event.send_message_to_source(e.parse_message_chain()).await?; 
        return Ok(true) 
      }

    }

    event
      .send_message_to_source("收到!".parse_message_chain())
      .await?;
    Ok(true)

  } else {
    Ok(false)
  }
}



/// 返回一个模块 (向过程宏改进中)
pub(crate) fn module() -> Module {
    // id, name, [plugins ...]
    module!("bot", "监听", bot)
}