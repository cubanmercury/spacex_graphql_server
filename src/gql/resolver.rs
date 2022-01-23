use crate::database::db::PgPool;
use crate::database::models::*;
use crate::database::services::*;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;
// use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{FieldResult, RootNode, EmptyMutation, EmptySubscription};
use std::sync::Arc;
// use tokio_postgres::{Client, NoTls};

struct Database {
  db: PgPool,
}
impl juniper::Context for Database {}

pub struct QueryRoot;
// struct MutationRoot;
// struct SubscriptionRoot;

#[juniper::graphql_object(context = Database,
  description = "Query Root",)]
impl QueryRoot {
  async fn company(ctx: &Database) -> FieldResult<UpdateCompanyGraphQL> {
    let conn = ctx.db.get()?;
    Ok(get_company_info(&conn).expect("Company info does not exist"))

    // let mut companies = Vec::new();
    // for row in rows {
    //   let company = CompanyGraphQL {
    //     id: row.try_get(0)?,
    //     name: row.try_get(1)?,
    //     founder: row.try_get(2)?,
    //     founded: row.try_get(3)?,
    //     employees: row.try_get(4)?,
    //     vehicles: row.try_get(5)?,
    //     launch_sites: row.try_get(6)?,
    //     test_sites: row.try_get(7)?,
    //     ceo: row.try_get(8)?,
    //     cto: row.try_get(9)?,
    //     coo: row.try_get(10)?,
    //     cto_propulsion: row.try_get(11)?,
    //     valuation: row.try_get(12)?,
    //     summary: row.try_get(13)?,
    //     headquarters_address: row.try_get(14)?,
    //     headquarters_city: row.try_get(15)?,
    //     headquarters_state: row.try_get(16)?,
    //     links_website: row.try_get(17)?,
    //     links_flickr: row.try_get(18)?,
    //     links_twitter: row.try_get(19)?,
    //     links_elon_twitter: row.try_get(20)?,
    //   };
    //   companies.push(company);
    // }
    // Ok(companies);
  }

}

// #[juniper::graphql_object(Context = Context)]
// impl MutationRoot {
//   async fn add_company(ctx: &Context, new_company: UpdateCompany) -> FieldResult<CompanyGraphQL> {
//     let id = uuid::Uuid::new_v4();

//     Ok(CompanyGraphQL {
//       id: id.to_string(),
//       name:
//     })
//   }
// }

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Database>, EmptySubscription<Database>>;



// pub async fn graphql(
//   schema: web::Data<Arc<Schema>>,
//   ctx: web::Data<Context>,
//   req: web::Json<GraphQLRequest>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//   // let res = req.execute(&schema, &ctx).await;
//   // let json = serde_json::to_string(&res).expect("Invalid JSON response");
//   // Ok(json)
// }

// graphql_object!(Query: () |&self|) {
//   field id() -> &str {
//     "1.0"
//   }
// }
