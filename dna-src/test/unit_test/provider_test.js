const sleep = require('sleep');

module.exports = (scenario) => {
  scenario.runTape('Provider Tests', (t, {liza}) => {
    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(4)
    const app_address = liza.call("provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    App_Details = {
      app_details:{
        details:"Details for this app",
        // domain_name:"app.holo.host"
      },
      app_hash:app_address.Ok
    }
    const app_details_address = liza.call("provider","add_app_details",App_Details);
    console.log("APP Details ADDRESS:: ",app_details_address);
    t.equal(app_details_address.Ok, 'QmXAYU3wHtnuuotABDY1WoqburChSseayBA2mkxWiw536P')

    sleep.sleep(5);

    const app_details_rec = liza.call("provider","get_app_details",{app_hash:app_address.Ok});
    console.log("Get Details:: ",app_details_rec);
    t.equal(app_details_rec.Ok[0].entry.details, "Details for this app")


    const my_apps = liza.call("provider","get_my_registered_app",{});
    console.log("my_apps:: ",my_apps);
    t.equal(my_apps.Ok.addresses.length, 1)


  })

scenario.runTape('Verify Provider', (t, {liza}) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    t.plan(2)
    const verified = liza.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);
    const is_verified = liza.call("provider", "is_registered_as_provider", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.addresses.length, 1)

  })
scenario.runTape('Provider Tests Domain Name', (t, {liza}) => {
    const App_Config = {
      ui_hash: "Quarnnnnvltuenbsfasf",
      dna_list: ["QweAFFRna","Qtavsvfava"]
    }
    t.plan(3)
    const app_address = liza.call("provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    // sleep.sleep(5);
    App_Domain_Name = {
      domain_name:"app2.holo.host",
      app_hash:app_address.Ok
    }
    const app_domain_name_address = liza.call("provider","add_app_domain_name",App_Domain_Name);
    console.log("APP Details ADDRESS:: ",app_domain_name_address);
    t.equal(app_domain_name_address.Ok, 'QmQ5QB4ZShmVgo8jkDs5XsJDGHrTZcnm7ULT9J2oH91qxT')

    sleep.sleep(5);

    const app_domain_name = liza.call("provider","get_app_domain_name",{app_hash:app_address.Ok});
    console.log("Get Domain Names:: ",app_domain_name);
    t.equal(app_domain_name.Ok[0].entry, '"app2.holo.host"')


  })
}
