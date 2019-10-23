const { one } = require('../config')

module.exports = (scenario) => {
  scenario('Provider Tests', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = await liza.call("app","provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)



    const App_Config = {
      app_bundle: {
        happ_hash: "QuarnnnnvltuenblergjasnvAfs"
      },
      domain_name: {
        dns_name: "app2.holo.host"
      }
    }
    const app_address = await liza.call("app","provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)



    const app_details_rec = await liza.call("app","provider","get_app_details",{app_hash:app_address.Ok});
    console.log("Get Details:: ",app_details_rec);
    t.equal(app_details_rec.Ok.app_bundle.happ_hash, "QuarnnnnvltuenblergjasnvAfs")

    const my_apps = await liza.call("app","provider","get_my_registered_app_list",{});
    console.log("my_apps:: ",my_apps);
    t.equal(my_apps.Ok.links.length, 1)

    await liza.kill()
  })

scenario('Verify Provider', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified = await liza.call("app","provider", "register_as_provider", Provider_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)



    const is_verified = await liza.call("app","provider", "is_registered_as_provider", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.links.length, 1)

    const HoloFuelAc={
      holofuel_account_details:{
        account_number:"Qnul------HF----------vn89a"
      }
    }

    let checking = await liza.call("app","provider", "get_holofuel_account", {});
    console.log("CHECK if Exists:: ",checking);
    t.equal(checking.Ok.links.length, 0)


    const HFC = await liza.call("app","provider", "add_holofuel_account", HoloFuelAc);
    console.log("HF COMMIT:: ",HFC);
    t.equal(HFC.Ok.length, 46)



    checking = await liza.call("app","provider", "get_holofuel_account", {});
    console.log("CHECK if Exists:: ",checking);
    t.equal(checking.Ok.links.length, 1)

    await liza.kill()
  })

scenario('Provider Tests Domain Name', async(s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const Provider_Doc = {
      provider_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    const verified_provider = await liza.call("app","provider", "register_as_provider", Provider_Doc);
    console.log("verified_provider:: ",verified_provider);
    t.equal(verified_provider.Ok.length, 46)



    const App_Config = {
      app_bundle: {
        happ_hash: "QuarnnnnvltuenblergjasnvAfs"
      },
      domain_name: {
        dns_name: "app2.holo.host"
      }
    }
    const app_address = await liza.call("app","provider", "register_app", App_Config);
    console.log("APP ADDRESS:: ",app_address);
    t.equal(app_address.Ok.length, 46)



    const app_domain_name = await liza.call("app","provider","get_app_domain_name",{app_hash:app_address.Ok});
    console.log("Get Domain Names:: ",app_domain_name);
    t.equal(app_domain_name.Ok[0].dns_name, 'app2.holo.host')

    await liza.kill()
  })
}
