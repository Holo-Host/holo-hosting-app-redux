module.exports = (scenario) => {
  scenario.runTape('get user address', async(t, {liza}) => {
    const result = liza.call("whoami", "get_user", {})
    console.log("->",result)
    t.equal(result.Ok.hash.length, 63) // agent addresses are 92 chars long?
  })
}
