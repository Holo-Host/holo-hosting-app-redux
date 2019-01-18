
const test = require('tape');

module.exports = (liza) => {
  test('Register App', (t) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(1)
    const app_address = liza.call("provider", "main", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address.Err);
    t.equal(app_address.Ok.length, 46)
  })
}