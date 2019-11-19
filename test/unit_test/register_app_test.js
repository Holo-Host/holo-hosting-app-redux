const { one } = require('../config')

const App_Config = {
  app_bundle: {
    happ_hash: "QuarnnnnvltuenblergjasnvAfs",
  },
  domain_name: {
    dns_name: "zs.com"
  }
}

const Provider_Config = {
  provider_doc: {
    kyc_proof: "Document Credentials: Qmasfjw2938riuoh"
  }
}

module.exports = (scenario) => {
  scenario('register app', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    let register_app_result = await liza.callSync("app","provider", "register_app", App_Config);
    console.log("register_app_result", register_app_result);
    t.equal(register_app_result.Err.Internal, 'Agent Not a Provider');

    const register_provider_result = await liza.callSync("app","provider", "register_as_provider", Provider_Config );
    console.log("App Address Hash: ", register_provider_result);
    t.equal(register_provider_result.Ok.length, 46);



    register_app_result = await liza.callSync("app","provider", "register_app", App_Config);
    console.log("App Address Hash: ", register_app_result);
    t.equal(register_app_result.Ok.length, 46);

    const Host_Doc = {
      host_doc: {
        kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
      }
    }

    const verified = await liza.callSync("app","host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    const all_apps_again = await liza.call("app","host","get_all_apps",{});
    console.log("All Apps: ",all_apps_again);
    t.equal(all_apps_again.Ok.length, 1)

    await liza.kill()
  })
}
