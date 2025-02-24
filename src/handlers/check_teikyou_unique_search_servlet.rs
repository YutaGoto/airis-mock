use axum::Form;

use crate::models::airis::TeikyouUniqueSearchServlet;
use crate::models::xml::Xml;

pub async fn check_teikyou_unique_search_servlet(
    body: Form<TeikyouUniqueSearchServlet>,
) -> Xml<TeikyouUniqueSearchServlet> {
    Xml(TeikyouUniqueSearchServlet {
        searchdate: body.searchdate.clone(),
        searchid: body.searchid.clone(),
        privacyflg: body.privacyflg.clone(),
        seqno: body.seqno.clone(),
        retryflg: body.retryflg.clone(),
        groupid: body.groupid.clone(),
        regno: body.regno.clone(),
        chassisno: body.chassisno.clone(),
        version: body.version.clone(),
        userid: body.userid.clone(),
        pw: body.pw.clone(),
        keyword: body.keyword.clone(),
    })
}
