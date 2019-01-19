const test = require('tape');

module.exports = (app) => {
  test('WhoAmI Test', (t) => {
    const result = app.call("whoami", "main", "get_user", {})
    t.equal(result.Ok.hash.length, 92) // agent addresses are 92 chars long?
    t.end()
  })
}
