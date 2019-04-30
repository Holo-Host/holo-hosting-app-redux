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
      app_bundle: {
        happ_hash: "QuarnnnnvltuenblergjasnvAfs",
        dna_list: ["QweAFioina","QtavsFdvva"]
      },
      app_details: {
        name:"App Name",
        details:"Details for this app",
      },
      domain_name: {
        dns_name: "app2.holo.host"
      }
    }

    const app_address = liza.call("provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    sleep.sleep(5);

    PaymentPref = {
      app_hash: app_address.Ok,
      max_fuel_per_invoice: 2.0,
      max_unpaid_value: 10,
    }

    const pref_commited = liza.call("host","add_service_log_details",PaymentPref);
    console.log("pref_commited:: ",pref_commited);
    t.equal(pref_commited.Ok, "QmdGAGUVSsFkYzNjvzxvC3F2kG5bgpmb1EzttWRfVXd3Vb")

    sleep.sleep(5);

    const app_bundle = liza.call("provider","get_app_details",{app_hash:app_address.Ok});
    // console.log("App_bundle:: ",app_bundle.Ok.app_bundle);
    // console.log("Payment_pref:: ",app_bundle.Ok.payment_pref);
    t.equal(app_bundle.Ok.app_bundle.happ_hash, App_Config.app_bundle.happ_hash)
    t.equal(app_bundle.Ok.payment_pref[0].entry.max_fuel_per_invoice, PaymentPref.max_fuel_per_invoice)


    const service_log_details = liza.call("host","get_service_log_details",{app_hash:app_address.Ok});
    console.log("SERVICe LOG Details: ",service_log_details);
    t.equal(service_log_details.Ok.max_unpaid_value, 10)


  })
}
