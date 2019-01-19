const test = require('tape');
const path = require('path');
const sleep = require('sleep');

// const { Config, Container } = require('@holochain/holochain-nodejs');
const { Config, Container } = require('../../holochain-rust/nodejs_container');

const dnaPath = "dist/bundle.json"

// IIFE to keep config-only stuff out of test scope
const container = (() => {
  const agentLiza = Config.agent("liza")
  // // const agentJack = Config.agent("jack")

  const dna = Config.dna(dnaPath)

  const instanceLiza = Config.instance(agentLiza, dna)
  // const instanceJack = Config.instance(agentJack, dna)

  // const containerConfig = Config.container([instanceLiza, instanceJack])
  const containerConfig = Config.container([instanceLiza])
  return new Container(containerConfig)
})()

// Initialize the Container
container.start()

const liza = container.makeCaller('liza', dnaPath)
// const jack = container.makeCaller('jack', dnaPath)

// test('agentId', (t) => {
//   t.plan(2)
//   t.ok(liza.agentId)
//   t.notEqual(liza.agentId, jack.agentId)
// })

require('./unit_test/whoami_test')(liza);
require('./unit_test/app_flow_test')(liza);
require('./unit_test/provider_test')(liza);
require('./unit_test/categories_test')(liza);
