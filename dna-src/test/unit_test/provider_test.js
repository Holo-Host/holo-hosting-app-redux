const sleep = require('sleep');

module.exports = (scenario) => {
  scenario.runTape('Provider Tests', async(t, {liza}) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = liza.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)

    sleep.sleep(5);

    const App_Config = {
      ui_hash: "QuarnnnnvltuenblergjasnvAfs",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    const app_address = liza.call("provider", "register_app_bundle", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    App_Details = {
      app_details:{
        name:"App Name",
        details:"Details for this app",
        // domain_name:"app.holo.host"
      },
      app_hash:app_address.Ok
    }
    const app_details_address = liza.call("provider","add_app_details",App_Details);
    console.log("APP Details ADDRESS:: ",app_details_address);
    t.equal(app_details_address.Ok, 'Qma8ZDMrav4zMsBLS25abKsxhGaPTiY8Efr8Tq7z6YxVad')

    sleep.sleep(5);

    const app_details_rec = liza.call("provider","get_app_details",{app_hash:app_address.Ok});
    console.log("Get Details:: ",app_details_rec);
    t.equal(app_details_rec.Ok.app_bundle.ui_hash, "QuarnnnnvltuenblergjasnvAfs")

    const my_apps = liza.call("provider","get_my_registered_app",{});
    console.log("my_apps:: ",my_apps);
    t.equal(my_apps.Ok.addresses.length, 1)


  })

scenario.runTape('Verify Provider', async(t, {liza}) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified = liza.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);

    const is_verified = liza.call("provider", "is_registered_as_provider", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.addresses.length, 1)

    const HoloFuelAc={
      holofuel_account_details:{
        account_number:"Qnul------HF----------vn89a"
      }
    }

    let checking = liza.call("provider", "get_holofuel_account", {});
    console.log("CHECK if Exists:: ",checking);
    t.equal(checking.Ok.addresses.length, 0)


    const HFC = liza.call("provider", "add_holofuel_account", HoloFuelAc);
    console.log("HF COMMIT:: ",HFC);
    t.equal(HFC.Ok.length, 46)

    sleep.sleep(5);

    checking = liza.call("provider", "get_holofuel_account", {});
    console.log("CHECK if Exists:: ",checking);
    t.equal(checking.Ok.addresses.length, 1)
  })

scenario.runTape('Provider Tests Domain Name', async(t, {liza}) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = liza.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)

    sleep.sleep(5);

    const App_Config = {
      ui_hash: "Quarnnnnvltuenbsfasf",
      dna_list: ["QweAFFRna","Qtavsvfava"]
    }
    const app_address = liza.call("provider", "register_app_bundle", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    App_Domain_Name = {
      domain_name:"app2.holo.host",
      app_hash:app_address.Ok
    }
    const app_domain_name_address = liza.call("provider","add_app_domain_name",App_Domain_Name);
    console.log("APP Details ADDRESS:: ",app_domain_name_address);
    t.equal(app_domain_name_address.Ok, 'QmPkuw9HB55FTnuWtjpsQDmxYjn1zxLyyegE4AJWdxoq4c')

    sleep.sleep(5);

    const app_domain_name = liza.call("provider","get_app_domain_name",{app_hash:app_address.Ok});
    console.log("Get Domain Names:: ",app_domain_name);
    t.equal(app_domain_name.Ok[0].entry.dns_name, 'app2.holo.host')
  })
}
