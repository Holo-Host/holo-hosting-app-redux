const sleep = require('sleep');

module.exports = (scenario) => {
  scenario.runTape('App Flow Test', async(t, {liza}) => {
    const App_Config_1 = {
      ui_hash: "Quarnnnnvltuenb###CONFIG1",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    const App_Config_2 = {
      ui_hash: "Quarnnnnvltuenb###CONFIG2",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }

    const app_address_1 = liza.call("provider",  "register_app", App_Config_1);
    const app_address_2 = liza.call("provider",  "register_app", App_Config_2);
    console.log("APP ADDRESS:: ",app_address_1);
    t.equal(app_address_1.Ok.length, 46)

    const Host_Doc = {
      host_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified = liza.call("host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);

    const app_enable = liza.call("host","enable_app",{app_hash:app_address_1.Ok});
    liza.call("host","enable_app",{app_hash:app_address_2.Ok});
    console.log("App_enbled: ", app_enable);
    t.equal(app_enable.Ok, null)

    sleep.sleep(5);

    const app_list = liza.call("host","get_enabled_app",{});
    console.log("APP List:: ",app_list);
    t.equal(app_list.Ok.length, 2)

    const agent_list = liza.call("host","get_host_for_app",{app_hash:app_address_2.Ok});
    console.log("Agent List:: ",agent_list.Ok[0]);
    t.equal(agent_list.Ok.length, 1)

    const disable_app_hash = liza.call("host","disable_app",{app_hash:app_address_2.Ok});
    console.log("App_Disabled: ", app_enable);
    t.equal(disable_app_hash.Ok, null)

    sleep.sleep(5);

    const app_list_after_disable = liza.call("host","get_enabled_app",{});
    console.log("APP list again:: ",app_list_after_disable);
    t.equal(app_list_after_disable.Ok.length, 1)

    sleep.sleep(5);

    /*
    Ok{
    recently_enabled_apps:[{app:"",host:[""]}],
    recently_disabled_apps:[{app:"",host:[""]}]
    }
    */
    const kv_updates = liza.call("host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates :: ",kv_updates.Ok.recently_enabled_apps[0]);
    // console.log("KV Updates Enabled App1:: ",kv_updates.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates.Ok.recently_enabled_apps[0].host.length, 1)
    t.equal(kv_updates.Ok.recently_disabled_apps.length, 2)

    sleep.sleep(5);
    /// Calling this fn again should return no new kvs
    const kv_updates_2 = liza.call("host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_2.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_2.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_2.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_2.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_2.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_2.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_2.Ok.recently_enabled_apps[0].host.length, 1)
    t.equal(kv_updates_2.Ok.recently_disabled_apps.length, 2)

    /// NOW the Updater responds that the KV has been updated
    sleep.sleep(5);
    liza.call("host","kv_updates_host_completed",{kv_bundle:kv_updates_2.Ok.recently_enabled_apps});

    sleep.sleep(5);
    /// Calling this fn again should return no new kvs
    const kv_updates_3 = liza.call("host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_3.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_3.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_3.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_3.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_3.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_3.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_3.Ok.recently_enabled_apps[0].host.length, 0)
    t.equal(kv_updates_3.Ok.recently_enabled_apps.length, 2)

    sleep.sleep(5);
    liza.call("host","kv_updates_host_completed",{kv_bundle:kv_updates_2.Ok.recently_disabled_apps});


    sleep.sleep(5);
    /// Calling this fn again should return no new kvs
    const kv_updates_4 = liza.call("host","get_kv_updates_dna_to_host",{});
    // console.log("KV Updates Enable:: ",kv_updates_4.Ok.recently_enabled_apps);
    // console.log("KV Updates Disabled:: ",kv_updates_4.Ok.recently_disabled_apps);
    // console.log("KV Updates Enabled App1:: ",kv_updates_4.Ok.recently_enabled_apps[0].host);
    // console.log("KV Updates Enabled App2:: ",kv_updates_4.Ok.recently_enabled_apps[1].host);
    // console.log("KV Updates Disabled App1:: ",kv_updates_4.Ok.recently_disabled_apps[0].host);
    // console.log("KV Updates Disabled App2:: ",kv_updates_4.Ok.recently_disabled_apps[1].host);
    t.equal(kv_updates_4.Ok.recently_disabled_apps[0].host.length, 0)
    t.equal(kv_updates_4.Ok.recently_disabled_apps.length, 2)

  })
}
