const test = require('tape');

module.exports = (liza) => {
  test('App Flow', (t) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(2)
    const app_address = liza.call("provider", "main", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    const App_Hash ={
      app_hash:app_address.Ok
    }
    const app_enable = liza.call("host","main","enable_app",App_Hash);
    console.log("APP Details ADDRESS:: ",app_enable);
    t.equal(app_enable.Ok, null)

  })
}
