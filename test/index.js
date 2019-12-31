//"use strict"; // locks up tests for some reason
// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape

/*
 * Try-o-rama
 */
const { Orchestrator, tapeExecutor, singleConductor, localOnly, combine, callSync } = require('@holochain/tryorama')

const MIN_EXPECTED_SCENARIOS = 1

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});


const networkType = process.env.APP_SPEC_NETWORK_TYPE || 'sim2h'
const middleware = 
  ( networkType === 'websocket'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim1h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim2h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'memory'
  ? combine(tapeExecutor(require('tape')), localOnly, singleConductor, callSync)

  : (() => {throw new Error(`Unsupported memory type: ${networkType}`)})()
)

const orchestrator = new Orchestrator({
  middleware,
  waiter: {
    softTimeout: 10000,
    hardTimeout: 20000
  }
})

require('./unit_test/whoami_test')(orchestrator.registerScenario);
require('./unit_test/app_flow_test')(orchestrator.registerScenario);
require('./unit_test/kv_enable_disable_test')(orchestrator.registerScenario);
require('./unit_test/dna_dns_test')(orchestrator.registerScenario);
require('./unit_test/host_test')(orchestrator.registerScenario);
require('./unit_test/provider_test')(orchestrator.registerScenario);
require('./unit_test/payment_prefs_test')(orchestrator.registerScenario);
require('./unit_test/retrive_all_apps')(orchestrator.registerScenario);
require('./unit_test/register_app_test')(orchestrator.registerScenario);


// Check to see that we haven't accidentally disabled a bunch of scenarios
const num = orchestrator.numRegistered()
if (num < MIN_EXPECTED_SCENARIOS) {
  console.error(`Expected at least ${MIN_EXPECTED_SCENARIOS} scenarios, but only ${num} were registered!`)
  process.exit(1)
}
else {
  console.log(`Registered ${num} scenarios (at least ${MIN_EXPECTED_SCENARIOS} were expected)`)
}

orchestrator.run().then(stats => {
  console.log("All done.")
})
