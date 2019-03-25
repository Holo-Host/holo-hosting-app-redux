const { Config, Scenario } = require("../../../holochain-rust/nodejs_conductor")
Scenario.setTape(require('tape'))
const dnaPath = "dist/dna-src.dna.json"
const dna = Config.dna(dnaPath, 'happs')
const agentLiza = Config.agent("liza")
const instanceLiza = Config.instance(agentLiza, dna)
const scenario = new Scenario([instanceLiza],{ debugLog: true })

// require('./unit_test/app_flow_test')(scenario);
// require('./unit_test/whoami_test')(scenario);
// require('./unit_test/dna_dns_test')(scenario);
// require('./unit_test/host_test')(scenario);
// require('./unit_test/provider_test')(scenario);
// require('./unit_test/kv_enable_disable_test')(scenario);
require('./unit_test/payment_prefs_test')(scenario);
// require('./unit_test/retrive_all_apps')(scenario);
