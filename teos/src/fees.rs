use lightning_invoice::Bolt11Invoice;

// WE PUT THIS FUNCTIONALITY BEHIND A TRAIT... SO THAT ULTIMATELY... WE CAN PUT ANY NODE IMPLEMENTATION BEHIND IT... LND, CLN... WHATEVER.
// The returned string is a BOLT 11 invoice.
//pub(crate) trait GetInvoice {
//    fn get_invoice(&self) -> Bolt11Invoice;
//}

pub trait ValidatePayment {
    // The returned string is a BOLT 11 invoice.
    fn get_invoice(&self) -> Bolt11Invoice;
    fn validate(&self) -> bool;
}

//pub mod clnrpc {
//    tonic::include_proto!("cln");
//}
//
//use std::path::PathBuf;
////use teos_common::protos::{Amount, InvoiceRequest, node_client::NodeClient};
//use tonic::Request;
//use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
//
//use crate::fees::clnrpc::{Amount, node_client::NodeClient};
//
//// NEED TO WRITE UP DOCUMENTATION OF FUCNTIONS I ASSUME.
//pub(crate) struct Cln {
//    client: NodeClient<tonic::transport::Channel>,
//}
//
//impl Cln {
//    // PASS IN THE DATA DIR HERE... AND OTHER CONFIG OPTIONS!
//    // Instructions: Should just write these in PR for now... ca.pem, client.pem and client-key.pem need to be copied over to `<DATA_DIR>/cln/`
//    async fn new(&self, data_dir: PathBuf) -> Self {
//        let data_dir_str = data_dir.into_os_string().into_string().unwrap();
//        println!("DATA_DIR_STR: {}", data_dir_str);
//
//        // MAKE SURE I"M DEALING WITH ALL THE UNWRAPS ADEQUATELY
//        let server_root_ca_cert = tokio::fs::read(format!("{data_dir_str}/cln/ca.pem")).await.unwrap();
//        let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
//        let client_cert = tokio::fs::read("{data_dir_str}/cln/client.pem").await.unwrap();
//        let client_key = tokio::fs::read("{data_dir_str}/cln/client-key.pem").await.unwrap();
//        let client_identity = Identity::from_pem(client_cert, client_key);
//
//        let tls = ClientTlsConfig::new()
//            .domain_name("localhost")
//            .ca_certificate(server_root_ca_cert)
//            .identity(client_identity);
//
//        let channel = Channel::from_static("http://localhost:10000")
//            .tls_config(tls)
//            .unwrap()
//            .connect()
//            .await
//            .unwrap();
//
//        let mut client = NodeClient::new(channel);
//        Cln{
//            client,
//        }
//    }
//}
//
//impl GetInvoice for Cln {
//    async fn get_invoice(&self) -> String {
//        let req = clnrpc::InvoiceRequest{
//	    amount_msat: Amount{msat: 1000},
//            ..Default::default()
//        };
//
//	let resp = self.client.Invoice(Request::new(req)).await.into_inner();
//        resp.bolt11
//    }
//}
