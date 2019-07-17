const path = require('path')
const tape = require('tape')

const { Orchestrator, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/try-o-rama')
const spawnConductor = require('./spawn_conductors')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/Holo-Hosting-App.dna.json")
const dna = Orchestrator.dna(dnaPath, 'hha')
// const dna2 = Orchestrator.dna(dnaPath, 'hha', {uuid: 'altered-dna'})

const commonConductorConfig = {
  instances: {
    app: dna,
  },
}

const orchestratorSimple = new Orchestrator({
  conductors: {
    liza: commonConductorConfig,
    // jack: commonConductorConfig,
  },
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

// const orchestratorMultiDna = new Orchestrator({
//   conductors: {
//     conductor: {
//       instances: {
//         app1: dna,
//         // app2: dna2,
//       },
//       bridges: [
//         Orchestrator.bridge('test-bridge', 'app1', 'app2')
//       ],
//     }
//   },
//   debugLog: false,
//   executor: tapeExecutor(require('tape')),
//   middleware: backwardCompatibilityMiddleware,
//   callbacksPort: 8888,
// })

require('./unit_test/whoami_test')(orchestratorSimple.registerScenario);
require('./unit_test/app_flow_test')(orchestratorSimple.registerScenario);
require('./unit_test/kv_enable_disable_test')(orchestratorSimple.registerScenario);
require('./unit_test/dna_dns_test')(orchestratorSimple.registerScenario);
require('./unit_test/host_test')(orchestratorSimple.registerScenario);
require('./unit_test/provider_test')(orchestratorSimple.registerScenario);
// require('./unit_test/payment_prefs_test')(orchestratorSimple.registerScenario);
// require('./unit_test/retrive_all_apps')(orchestratorSimple.registerScenario);
// require('./unit_test/register_app_test')(orchestratorSimple.registerScenario);

// require('./multi-dna')(orchestratorMultiDna.registerScenario)

const run = async () => {
  const liza = await spawnConductor('liza', 3000)
  await orchestratorSimple.registerConductor({name: 'liza', url: 'http://0.0.0.0:3000'})
  // const jack = await spawnConductor('jack', 4000)
  // await orchestratorSimple.registerConductor({name: 'jack', url: 'http://0.0.0.0:4000'})

  const delay = ms => new Promise(resolve => setTimeout(resolve, ms))
  console.log("Waiting for conductors to settle...")
  await delay(5000)
  console.log("Ok, starting tests!")

  await orchestratorSimple.run()
  liza.kill()
  // jack.kill()

  // Multi instance tests where n3h is the network connecting them currently fails with the 2nd instance
  // waiting for and not receiving the agent entry of the first one.
  // I believe this is due to n3h not sending a peer connected message for a local instance
  // and core has not implented the authoring list yet...
  //const conductor = await spawnConductor('conductor', 6000)
  //await orchestratorMultiDna.registerConductor({name: 'conductor', url: 'http://0.0.0.0:6000'})
  //await orchestratorMultiDna.run()
  //conductor.kill()

  process.exit()
}

run()
