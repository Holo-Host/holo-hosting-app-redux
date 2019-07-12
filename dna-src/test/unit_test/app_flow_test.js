module.exports = (scenario) => {
  scenario('App Flow Test',async (s, t, {liza}) => {
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
    const verified = await liza.app.call("host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)
    const Provider_Doc = {
      provider_doc:{
        kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
      }
    }
    const verified_provider = await liza.app.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)

    const app_address_1 = await liza.app.call("provider",  "register_app", App_Config_1);
    const app_address_2 = await liza.app.call("provider",  "register_app", App_Config_2);
    console.log("APP ADDRESS 1:: ",app_address_1);
    t.equal(app_address_1.Ok, "QmQHz2S91HygBTqJmLjPCSTSyx5BYC3yidnyTrjVew8AxY");

    const app_enable = await liza.app.call("host","enable_app",{app_hash:app_address_1.Ok});
    await liza.app.call("host","enable_app",{app_hash:app_address_2.Ok});
    t.equal(app_enable.Ok, null)

    const app_list = await liza.app.call("host","get_enabled_app_list",{});
    console.log("APP List:: ",app_list);
    t.equal(app_list.Ok.length, 2)

    const agent_list = await liza.app.call("host","get_host_for_app",{app_hash:app_address_2.Ok});
    console.log("Agent List:: ",agent_list);
    t.equal(agent_list.Ok.length, 1)

    const disable_app_hash = await liza.app.call("host","disable_app",{app_hash:app_address_2.Ok});
    t.equal(disable_app_hash.Ok, null)

    const app_list_after_disable = await liza.app.call("host","get_enabled_app_list",{});
    console.log("APP list again:: ",app_list_after_disable);
    t.equal(app_list_after_disable.Ok.length, 1)

  })
}
