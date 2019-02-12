
module.exports = (scenario) => {
  scenario.runTape('get user address', (t, {alice}) => {
    const result = alice.call("whoami", "get_user", {})
    console.log(result)
    t.equal(result.Ok.hash.length, 92) // agent addresses are 92 chars long?
  })
}
