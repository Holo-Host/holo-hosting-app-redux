
const test = require('tape');

module.exports = (liza) => {
  test('Register App', (t) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(2)
    const app_address = liza.call("provider", "main", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    App_Details = {
      app_details:{
        details:"Details for this app"
      },
      app_hash:app_address.Ok
    }
    const app_details_address = liza.call("provider","main","add_app_details",App_Details);
    console.log("APP Details ADDRESS:: ",app_details_address);
    t.equal(app_details_address.Ok, 'QmXAYU3wHtnuuotABDY1WoqburChSseayBA2mkxWiw536P')

  })
}
