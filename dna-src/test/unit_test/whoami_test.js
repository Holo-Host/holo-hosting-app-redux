module.exports = (scenario) => {
  scenario('get user address', async(s, t, {liza}) => {
    const result = await liza.call("whoami", "get_user", {})
    console.log("->",result)
    t.equal(result.Ok.hash.length, 63) // agent addresses are 92 chars long?
  })
}
