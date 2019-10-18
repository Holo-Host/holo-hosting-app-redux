const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/try-o-rama')

const dnaPath = path.join(__dirname, "../dist/holo-hosting-app.dna.json")
const dna = Config.dna(dnaPath, 'holo-hosting-app')

const one = (agent) => ({
    instances: [{
	id: 'app',
	agent: {
	    id: agent,
	    name: `${agent}`,
	    test_agent: true,
	    public_address: "",
	    keystore_file: ""
	},
	dna: {
	    id: 'holo-hosting-app',
	    file: dnaPath,
	}
    }]
})

const callSyncMiddleware = (run, f) => run(s => {
  const s_ = Object.assign({}, s, {
    players: async (...a) => {
      const players = await s.players(...a)
      const players_ = _.mapValues(
        players,
        api => Object.assign(api, {
          callSync: async (...b) => {
            const result = await api.call(...b)
            await s.consistency()
            return result
          }
        })
      )
      return players_
    }
  })
  return f(s_)
})


module.exports = { one, callSyncMiddleware }
