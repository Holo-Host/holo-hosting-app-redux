const { Config, Container, Scenario } = require('../../../holochain-rust/nodejs_conductor')
Scenario.setTape(require('tape'))
const dnaPath = "dist/bundle.json"
const dna = Config.dna(dnaPath, 'happs')
const agentAlice = Config.agent("alice")
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

require('./unit_test/whoami_test')(scenario);
// require('./single_agent_tests/categories_test')(scenario);
// require('./single_agent_tests/ratings_test')(scenario);
// require('./single_agent_tests/whoami_test')(scenario);
