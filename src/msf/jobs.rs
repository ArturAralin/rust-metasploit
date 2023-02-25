//! A module to handle the jobs in Metasploit
use crate::client::Client;
use crate::connect::connect_async;
use crate::error::{MsfError, RpcError};
use crate::structs::{request as req, response as res};
use rmp_serde::{
  decode::{from_read, Error as derror},
  Deserializer, Serializer,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// To list all the currently running jobs
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,job};
/// use std::collections::HashMap;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:HashMap<String,String>=jobs::list(client.clone()).await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn list(client: Client) -> Result<HashMap<String, String>, MsfError> {
  let mut test: Result<HashMap<String, String>, MsfError> = Ok(HashMap::new());
  let mut body = Vec::new();
  let mut buf = vec![];
  let mut se = Serializer::new(&mut body);
  let byte = req::jobs::list("job.list".to_string(), client.token.unwrap());
  byte.serialize(&mut se).unwrap();
  let con = connect_async(client.url, body, &mut buf).await;
  let new_buf = buf.clone();
  let mut de = Deserializer::new(new_buf.as_slice());
  match con {
    Ok(_) => {
      let de_ret: Result<HashMap<String, String>, derror> = Deserialize::deserialize(&mut de);
      if let Ok(ref val) = de_ret {
        test = Ok(val.clone());
      };
      if let Err(_) = de_ret {
        let de_ret: MsfError = from_read(new_buf.as_slice()).unwrap();
        test = Err(de_ret);
      };
    }
    Err(_) => {
      panic!("Connection closed unexpectedly");
    }
  }
  test
}
/// To get information about the specified job
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,jobs};
/// use metasploit::response::jobs as resp;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Client::new("127.0.0.1",55552,"msf","password",true);
///     let response:resp::info=jobs::info(client.clone(),"1").await.unwrap();
///     println!("{:?}",response);
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn info(client: Client, job_id: u32) -> Result<res::jobs::info, RpcError> {
  let job_id: String = job_id.to_string();

  let mut test: Result<res::jobs::info, RpcError> = Ok(res::jobs::info {
    jid: 0,
    start_time: 0,
    name: String::new(),
    uripath: String::new(),
    datastore: Default::default(),
  });

  let body = rmp_serde::encode::to_vec(&req::jobs::info(
    "job.info".to_string(),
    client.token.unwrap(),
    job_id,
  ))?;

  let mut result = vec![];

  match connect_async(client.url, body, &mut result).await {
    Ok(_) => {
      let x: rmpv::Value = rmp_serde::decode::from_slice(&result)?;

      let r = rmp_serde::decode::from_slice::<res::jobs::info>(&result)?;

      // println!("{:?}", r);

      println!("{:#?}", x);
      // let de_ret: Result<res::jobs::info, derror> = Deserialize::deserialize(&mut de);
      // if let Ok(ref val) = de_ret {
      //   test = Ok(val.clone());
      // };

      // if let Err(_) = de_ret {
      //   // let de_ret: MsfError = from_read(new_buf.as_slice()).unwrap();
      //   // test = Err(de_ret);
      // };
    }
    Err(_) => {
      panic!("Connection closed unexpectedly");
    }
  }

  test
}
/// To stop a specified job
///
/// ## Example
/// ```
/// use metasploit::client::Client;
/// use metasploit::msf::{auth,jobs};
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(),Error> {
///     let client=Cluent::mew("127.0.0.1",55552,"msf","password",true);
///     assert_eq!(true,jobs::stop(client.clone(),"1").await.unwrap());
///     auth::logout(client.clone()).await.unwrap();
/// }
/// ```
pub async fn stop(client: Client, jobidstr: &str) -> Result<bool, MsfError> {
  let jobid: String = jobidstr.to_string();
  let mut test: Result<bool, MsfError> = Ok(false);
  let mut body = Vec::new();
  let mut buf = vec![];
  let mut se = Serializer::new(&mut body);
  let byte = req::jobs::stop("job.stop".to_string(), client.token.unwrap(), jobid);
  byte.serialize(&mut se).unwrap();
  let con = connect_async(client.url, body, &mut buf).await;
  let new_buf = buf.clone();
  let mut de = Deserializer::new(new_buf.as_slice());
  match con {
    Ok(_) => {
      let de_ret: Result<res::jobs::stop, derror> = Deserialize::deserialize(&mut de);
      if let Ok(ref val) = de_ret {
        if val.result == "success".to_string() {
          test = Ok(true);
        } else {
          test = Ok(false);
        }
      };
      if let Err(_) = de_ret {
        let de_ret: MsfError = from_read(new_buf.as_slice()).unwrap();
        test = Err(de_ret);
      };
    }
    Err(_) => {
      panic!("Connection closed unexpectedly");
    }
  }
  test
}
