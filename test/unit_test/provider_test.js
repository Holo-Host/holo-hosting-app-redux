const sleep = require('sleep');
const test = require('tape');

module.exports = (liza) => {
  test('Provider Tests', (t) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(3)
    const app_address = liza.call("provider", "main", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    App_Details = {
      app_details:{
        details:"Details for this app",
        domain_name:"app.holo.host"
      },
      app_hash:app_address.Ok
    }
    const app_details_address = liza.call("provider","main","add_app_details",App_Details);
    console.log("APP Details ADDRESS:: ",app_details_address);
    t.equal(app_details_address.Ok, 'QmaVKt7iMmFgoYgm5fLdZNFbV5RZ12AhSBX8qhZo7hjj4S')

    const my_apps = liza.call("provider","main","get_my_registered_app",{});
    console.log("my_apps:: ",my_apps);
    t.equal(my_apps.Ok.addresses.length, 3)


  })

  test('Verify Provider', (t) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    t.plan(2)
    const verified = liza.call("provider", "main", "register_as_provider", Provider_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);
    const is_verified = liza.call("provider", "main", "is_registered_as_provider", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.addresses.length, 1)

  })
}
