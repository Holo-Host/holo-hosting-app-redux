const sleep = require('sleep');

module.exports = (scenario) => {
scenario.runTape('DNS TO DNA Tests', async(t, {liza}) => {
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
        ui_hash: "Quarnnnnvltuenbsfasf",
        dna_list: ["QweAFioina","QtavsFdvva"]
      },
      app_details: {
        name: "App DNS Test",
        details: "Details for the world to know about the App DNS Test."
      },
      domain_name: {
        dns_name: "appDNS1.holo.host"
      }
    }
    const app_address = liza.call("provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    const app_domain_name = liza.call("provider","get_app_domain_name",{app_hash:app_address.Ok});
    console.log("Get Domain Names:: ",app_domain_name);
    t.equal(app_domain_name.Ok[0].entry.dns_name, 'appDNS1.holo.host')

    // Get the new DNS<=>DNAs
    // Return VAlues to the KV store
    // [{
    //   dna:[],
    //   dns:{
    //     address:"",
    //     entry:""
    //   }
    // }]
    const new_domain_name = liza.call("provider","get_kv_updates_domain_name",{});
    console.log("New Domain Names KVs:: ",new_domain_name);
    console.log("New Domain Names KVs:: ",new_domain_name.Ok[0].dns);
    t.equal(new_domain_name.Ok[0].dns[0].name, 'appDNS1.holo.host')

    // const new_domain_name_again = liza.call("provider","get_kv_updates_domain_name",{});
    // console.log("New Domain Names KVs:: ",new_domain_name_again);
    // console.log("New Domain Names KVs:: ",new_domain_name_again.Ok[0].dns);
    // t.equal(new_domain_name_again.Ok[0].dns.length, 0)
    //

    // delete the updated DNS details are recieved
    console.log("SENDING... :",new_domain_name.Ok);
    const updated = liza.call("provider","kv_updates_domain_name_completed",{kv_bundle:new_domain_name.Ok});
    console.log("Is Updated?:: ",updated);
    t.equal(updated.Ok, null)

    const again_new_domain_name = liza.call("provider","get_kv_updates_domain_name",{});
    console.log("Checking New Domain Names After Deleting:: ",again_new_domain_name);
    t.equal(again_new_domain_name.Ok.length, 1)
  })
}
