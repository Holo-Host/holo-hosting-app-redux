const { one } = require('../config')

module.exports = (scenario) => {
  scenario('get user address', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const result = await liza.call("app","whoami", "get_user", {})
    console.log("->",result)
    t.ok(result.Ok) // agent addresses are 92 chars long?

    await liza.kill()
  })
}
