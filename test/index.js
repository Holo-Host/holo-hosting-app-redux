//"use strict"; // locks up tests for some reason
// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape

/*
 * Try-o-rama
 */
const { Orchestrator, tapeExecutor, Config, singleConductor, combine, callSync, localOnly } = require('@holochain/tryorama')

const MIN_EXPECTED_SCENARIOS = 1

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dumbWaiter = interval => (run, f) => run(s =>
  f(Object.assign({}, s, {
    consistency: () => new Promise(resolve => {
      console.log(`dumbWaiter is waiting ${interval}ms...`)
      setTimeout(resolve, interval)
    })
  }))
)


let transport_config = 'memory';
let middleware = combine(
  // by default, combine conductors into a single conductor for in-memory networking
  // NB: this middleware makes a really huge difference! and it's not very well tested,
  // as of Oct 1 2019. So, keep an eye out.
  tapeExecutor(require('tape')),
  localOnly,
  callSync
);


const orchestrator = new Orchestrator({
    middleware,
    waiter: {
	softTimeout: 5000,
	hardTimeout: 10000,
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
