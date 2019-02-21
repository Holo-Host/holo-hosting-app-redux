const sleep = require('sleep');

module.exports = (scenario) => {

scenario.runTape('DNS TO DNA Tests', (t, {liza}) => {
    const App_Config = {
      ui_hash: "Quarnnnnvltuenbsfasf",
      dna_list: ["QweAFFRna","Qtavsvfava"]
    }
    // t.plan(3)
    const app_address = liza.call("provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)

    // sleep.sleep(5);
    App_Domain_Name1 = {
      domain_name:"app2.holo.host",
      app_hash:app_address.Ok
    }
    const app_domain_name_address = liza.call("provider","add_app_domain_name",App_Domain_Name1);
    console.log("APP Domain Details ADDRESS:: ",app_domain_name_address);
    t.equal(app_domain_name_address.Ok, 'QmPkuw9HB55FTnuWtjpsQDmxYjn1zxLyyegE4AJWdxoq4c')
    App_Domain_Name2 = {
      domain_name:"app2.org",
      app_hash:app_address.Ok
    }
  liza.call("provider","add_app_domain_name",App_Domain_Name2);
    // console.log("APP Domain Details ADDRESS:: ",app_domain_name_address);
    // t.equal(app_domain_name_address.Ok, 'QmQ5QB4ZShmVgo8jkDs5XsJDGHrTZcnm7ULT9J2oH91qxT')

    sleep.sleep(5);

    const app_domain_name = liza.call("provider","get_app_domain_name",{app_hash:app_address.Ok});
    console.log("Get Domain Names:: ",app_domain_name);
    t.equal(app_domain_name.Ok[0].entry.dns_name, 'app2.holo.host')

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
    t.equal(new_domain_name.Ok[0].dns[0].name, 'app2.holo.host')


    sleep.sleep(5);

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

    sleep.sleep(5);

    const again_new_domain_name = liza.call("provider","get_kv_updates_domain_name",{});
    console.log("Checking New Domain Names After Deleting:: ",again_new_domain_name);
    t.equal(again_new_domain_name.Ok.length, 1)



  })
}
