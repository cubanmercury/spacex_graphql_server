use super::models::*;
use super::schema::company_info::dsl::*;

use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn get_company_info(conn: &PgConnection) -> QueryResult<UpdateCompanyGraphQL> {
  let company = company_info.load::<UpdateCompanyGraphQL>(conn)?;

  company
}
