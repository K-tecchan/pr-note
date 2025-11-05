use graphql_client::GraphQLQuery;

#[allow(dead_code)]
type GitObjectID = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/graphql/schema.graphql",
    query_path = "src/github/graphql/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct GetUnMergedCommits;
