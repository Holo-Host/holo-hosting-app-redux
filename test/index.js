const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/Holo-Hosting-App.dna.json")
const dna = Diorama.dna(dnaPath, 'hha')

const diorama = new Diorama({
  instances: {
    liza: dna,
    // jack: dna,
  },
  // bridges: [
  //   Diorama.bridge('test-bridge', 'liza', 'jack')
  // ],
  debugLog: true,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

// require('./unit_test/whoami_test')(diorama.registerScenario);
// require('./unit_test/app_flow_test')(diorama.registerScenario);
// require('./unit_test/kv_enable_disable_test')(diorama.registerScenario);
// require('./unit_test/dna_dns_test')(diorama.registerScenario);
// require('./unit_test/host_test')(diorama.registerScenario);
// require('./unit_test/provider_test')(diorama.registerScenario);
// require('./unit_test/payment_prefs_test')(diorama.registerScenario);
// require('./unit_test/retrive_all_apps')(diorama.registerScenario);
require('./unit_test/register_app_test')(diorama.registerScenario);

diorama.run()
