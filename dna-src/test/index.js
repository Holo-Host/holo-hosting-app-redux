const { Config, Container, Scenario } = require('../../../holochain-rust/nodejs_conductor')
Scenario.setTape(require('tape'))
const dnaPath = "dist/bundle.json"
const dna = Config.dna(dnaPath, 'happs')
const agentLiza = Config.agent("liza")
const instanceLiza = Config.instance(agentLiza, dna)
const scenario = new Scenario([instanceLiza])

// require('./unit_test/app_flow_test')(scenario);
// require('./unit_test/whoami_test')(scenario);
// require('./unit_test/host_test')(scenario);
// require('./unit_test/provider_test')(scenario);
require('./unit_test/dna_dns_test')(scenario);
