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
        details:"Details for this app"
        // domain_name:"app.holo.host"
      },
      app_hash:app_address.Ok
    }
    const app_details_address = liza.call("provider","add_app_details",App_Details);
    console.log("APP Details ADDRESS:: ",app_details_address);
    t.equal(app_details_address.Ok, 'Qma8ZDMrav4zMsBLS25abKsxhGaPTiY8Efr8Tq7z6YxVad')

    sleep.sleep(5);

    PaymentPref = {
      app_hash: app_address.Ok,
      max_fuel_per_invoice: 2.0,
      max_unpaid_value: 10,
    }

    const pref_commited = liza.call("provider","add_service_log_details",PaymentPref);
    console.log("pref_commited:: ",pref_commited);
    t.equal(pref_commited.Ok, "QmZZL2W8R1WghusNw9ZEUsWJfvj69KmnTrqCW2HoUu8i6p")

    sleep.sleep(5);

    const app_bundle = liza.call("provider","get_app_details",{app_hash:app_address.Ok});
    console.log("App_bundle:: ",app_bundle.Ok.app_bundle);
    console.log("App_details:: ",app_bundle.Ok.app_details);
    console.log("Payment_pref:: ",app_bundle.Ok.payment_pref);
    t.equal(app_bundle.Ok.app_bundle.ui_hash, App_Config.ui_hash)
    t.equal(app_bundle.Ok.app_details[0].entry.details, App_Details.app_details.details)
    t.equal(app_bundle.Ok.payment_pref[0].entry.max_fuel_per_invoice, PaymentPref.max_fuel_per_invoice)

  })
}
