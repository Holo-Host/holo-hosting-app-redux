
const test = require('tape');

module.exports = (liza,jack) => {
  test('Register App', (t) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(1)
    const app_address = liza.call("provider", "main", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.length, 46)
  })
}