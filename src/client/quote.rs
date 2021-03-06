use failure::Error;
use futures::Future;

use error::Result;
use model::quote::{GetQuoteBucketedRequest, GetQuoteBucketedResponse, GetQuoteRequest, GetQuoteResponse};

use super::BitMEX;

impl BitMEX {
    pub fn get_quote(&self, req: GetQuoteRequest) -> Result<impl Future<Item = Vec<GetQuoteResponse>, Error = Error>> {
        Ok(self.transport.get("/quote", Some(req))?)
    }
    pub fn get_quote_bucketed(&self, req: GetQuoteBucketedRequest) -> Result<impl Future<Item = Vec<GetQuoteBucketedResponse>, Error = Error>> {
        Ok(self.transport.get("/quote/bucketed", Some(req))?)
    }
}
