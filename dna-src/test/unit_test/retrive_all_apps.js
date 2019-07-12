const sleep = require('sleep');
module.exports = (scenario) => {
  scenario('App Flow Test',async (s, t, {liza}) => {
    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = await liza.call("provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)

    sleep.sleep(5);

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

    const app_address_1 = await liza.call("provider",  "register_app", App_Config_1);
    const app_address_2 = await liza.call("provider",  "register_app", App_Config_2);
    console.log("APP ADDRESS:: ",app_address_1);
    t.equal(app_address_1.Ok, "QmQHz2S91HygBTqJmLjPCSTSyx5BYC3yidnyTrjVew8AxY");

    sleep.sleep(5);

    const all_apps = await liza.call("host","get_all_apps",{});
    console.log("All Apps: ",all_apps);
    t.equal(all_apps.Err.Internal, 'Agent Not a Host')

    const Host_Doc = {
      host_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}

    const verified = await liza.call("host", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);

    const all_apps_again = await liza.call("host","get_all_apps",{});
    console.log("All Apps: ",all_apps_again);
    t.equal(all_apps_again.Ok.length, 2)
})
}
