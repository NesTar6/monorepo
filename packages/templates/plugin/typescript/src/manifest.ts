import { PluginManifest } from "@web3api/core-js";

export const manifest: PluginManifest = {
  // TODO: use the schema.graphql
  // https://github.com/web3-api/monorepo/issues/101
  schema: `
type Query {
  sampleQuery(data: String!): String!
}

type Mutation {
  sampleMutation(data: Bytes!): Boolean!
}`,
  implemented: [],
  imported: [],
};
