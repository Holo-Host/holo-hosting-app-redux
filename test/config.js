const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaName = "holo-hosting-app"
const dnaId = "holo-hosting-app"

const dnaPath = path.join(__dirname, `../dist/${dnaName}.dna.json`)
const dna = Config.dna(dnaPath, dnaId)


const networkType = process.env.APP_SPEC_NETWORK_TYPE || "sim2h"
let network = {}
// override the transport_config if we are in the Final Exam context!
if (process.env.HC_TRANSPORT_CONFIG) {
    network=require(process.env.HC_TRANSPORT_CONFIG)
} else {
    network =
        ( networkType === 'websocket'
          ? Config.network('websocket')

          : networkType === 'memory'
          ? Config.network('memory')

          : networkType === 'sim1h'
          ? {
              type: 'sim1h',
              dynamo_url: 'http://localhost:8000'
          }

          : networkType === 'sim2h'
          ? {
              type: 'sim2h',
              sim2h_url: 'wss://localhost:9000'
          }

          : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
        )
}

const logger = {
  type: 'debug',
  rules: {
    rules: [
      {
        exclude: true,
        pattern: '.*parity.*'
      },
      {
        exclude: true,
        pattern: '.*mio.*'
      },
      {
        exclude: true,
        pattern: '.*tokio.*'
      },
      {
        exclude: true,
        pattern: '.*hyper.*'
      },
      {
        exclude: true,
        pattern: '.*rusoto_core.*'
      },
      {
        exclude: true,
        pattern: '.*want.*'
      },
      {
        exclude: true,
        pattern: '.*rpc.*'
      }
    ]
  },
  state_dump: true
    // dpki: {
    //   instance_id: 'dpki_happ',
    //   init_params: {"revocation_key": "HcSCiPdMkst9geux7y7kPoVx3W54Ebwkk6fFWjH9V6oIbqi77H4i9qGXRsDcdbi","signed_auth_key":"zJkRXrrbvbzbH96SpapO5lDWoElpzB1rDE+4zbo/VthM/mp9qNKaVsGiVKnHkqT4f5J4MGN+q18xP/hwQUKyDA=="}
    // },
}

const commonConfig = { logger, network }

module.exports = {
    one: (agent) => Config.gen([{
	id: 'app',
	agent: {
	    id: agent,
	    name: `${agent}`,
	    test_agent: true,
	    public_address: "",
	    keystore_file: ""
	},
	dna: {
	    id: dnaId,
	    file: dnaPath,
	}
    }],
    commonConfig
  )
}
