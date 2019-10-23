const { one } = require('../config')

module.exports = (scenario) => {
  scenario('App Flow Test', async (s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const App_Config_1 = {
      app_bundle: {
        happ_hash: "Quarnnnnvltuenb###CONFIG1"
      },
      domain_name: {
        dns_name: "apptest1.com"
      }
    }
    const App_Config_2 = {
      app_bundle: {
        happ_hash: "Quarnnnnvltuenb###CONFIG2"
      },
      domain_name: {
        dns_name: "apptest2.com"
      }
    }
    const Host_Doc = {
      host_doc:{
        kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
      }
    }
    const verified = await liza.call("app","host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.ok(verified.Ok)
    const Provider_Doc = {
      provider_doc:{
        kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
      }
    }
    const verified_provider = await liza.call("app","provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.ok(verified_provider.Ok)

    const app_address_1 = await liza.call("app", "provider", "register_app", App_Config_1);
    const app_address_2 = await liza.call("app","provider",  "register_app", App_Config_2);
    console.log("APP ADDRESS 1:: ",app_address_1);
    t.ok(app_address_1.Ok);

    const app_enable = await liza.call("app","host","enable_app",{app_hash:app_address_1.Ok});
    await liza.call("app","host","enable_app",{app_hash:app_address_2.Ok});
    t.equal(app_enable.Ok, null)

    const app_list = await liza.call("app","host","get_enabled_app_list",{});
    console.log("APP List:: ",app_list);
    t.equal(app_list.Ok.length, 2)

    const agent_list = await liza.call("app","host","get_host_for_app",{app_hash:app_address_2.Ok});
    console.log("Agent List:: ",agent_list);
    t.equal(agent_list.Ok.length, 1)

    const disable_app_hash = await liza.call("app","host","disable_app",{app_hash:app_address_2.Ok});
    t.equal(disable_app_hash.Ok, null)

    const app_list_after_disable = await liza.call("app","host","get_enabled_app_list",{});
    console.log("APP list again:: ",app_list_after_disable);
    t.equal(app_list_after_disable.Ok.length, 1)

    await liza.kill()
  })
}
