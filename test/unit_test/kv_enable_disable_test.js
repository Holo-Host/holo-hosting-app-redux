const { one } = require('../config')

module.exports = (scenario) => {
  scenario('App Flow Test', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = await liza.call("app","provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)



    const App_Config_1 = {
      app_bundle: {
        happ_hash: "Quarnnnnvltuenb###CONFIG1",
      },
      domain_name: {
        dns_name: "apptest1.com"
      }
    }
    const App_Config_2 = {
      app_bundle: {
        happ_hash: "Quarnnnnvltuenb###CONFIG2",
      },
      domain_name: {
        dns_name: "apptest2.com"
      }
    }

    const app_address_1 = await liza.call("app","provider",  "register_app", App_Config_1);
    const app_address_2 = await liza.call("app","provider",  "register_app", App_Config_2);
    console.log("APP ADDRESS:: ",app_address_1);
    t.equal(app_address_1.Ok.length, 46)

    const Host_Doc = {
      host_doc:{
        kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
      }
    }
    const verified = await liza.call("app","host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)



    const app_enable = await liza.call("app","host","enable_app",{app_hash:app_address_1.Ok});
    await liza.call("app","host","enable_app",{app_hash:app_address_2.Ok});
    console.log("App_enbled: ", app_enable);
    t.equal(app_enable.Ok, null)



    const app_list = await liza.call("app","host","get_enabled_app_list",{});
    console.log("APP List:: ",app_list);
    t.equal(app_list.Ok.length, 2)

    const agent_list = await liza.call("app","host","get_host_for_app",{app_hash:app_address_2.Ok});
    console.log("Agent List:: ",agent_list.Ok[0]);
    t.equal(agent_list.Ok.length, 1)

    const disable_app_hash = await liza.call("app","host","disable_app",{app_hash:app_address_2.Ok});
    console.log("App_Disabled: ", app_enable);
    t.equal(disable_app_hash.Ok, null)



    const app_list_after_disable = await liza.call("app","host","get_enabled_app_list",{});
    console.log("APP list again:: ",app_list_after_disable);
    t.equal(app_list_after_disable.Ok.length, 1)



    /*
    Ok{
    recently_enabled_apps:[{app:"",host:[""]}],
    recently_disabled_apps:[{app:"",host:[""]}]
    }
    */
    const kv_updates = await liza.call("app","host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates :: ",kv_updates.Ok.recently_enabled_apps[0]);
    // console.log("KV Updates Enabled App1:: ",kv_updates.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates.Ok.recently_enabled_apps[0].host.length, 1)
    t.equal(kv_updates.Ok.recently_disabled_apps.length, 2)


    /// Calling this fn again should return no new kvs
    const kv_updates_2 = await liza.call("app","host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_2.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_2.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_2.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_2.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_2.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_2.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_2.Ok.recently_enabled_apps[0].host.length, 1)
    t.equal(kv_updates_2.Ok.recently_disabled_apps.length, 2)

    /// NOW the Updater responds that the KV has been updated

    await liza.call("app","host","kv_updates_host_completed",{kv_bundle:kv_updates_2.Ok.recently_enabled_apps});


    /// Calling this fn again should return no new kvs
    const kv_updates_3 = await liza.call("app","host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_3.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_3.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_3.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_3.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_3.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_3.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_3.Ok.recently_enabled_apps[0].host.length, 0)
    t.equal(kv_updates_3.Ok.recently_enabled_apps.length, 2)


    await liza.call("app","host","kv_updates_host_completed",{kv_bundle:kv_updates_2.Ok.recently_disabled_apps});



    /// Calling this fn again should return no new kvs
    const kv_updates_4 = await liza.call("app","host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_4.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_4.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_4.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_4.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_4.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_4.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_4.Ok.recently_disabled_apps[0].host.length, 0)
    t.equal(kv_updates_4.Ok.recently_disabled_apps.length, 2)
    await liza.kill()

  })
}
