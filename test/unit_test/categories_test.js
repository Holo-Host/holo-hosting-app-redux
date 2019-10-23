const test = require('tape');
const { one } = require('../config')

const App_Config_1 = {
  app_bundle: {
    happ_hash: "QuarnnnnvltuenblergjasnvAfs",
  },
  domain_name: {
    dns_name: "apptest1.com"
  }
}
const App_Config_2 = {
  app_bundle: {
    happ_hash: "QrafVSdvzv98vlsurhvsdfvser",
  },
  domain_name: {
    dns_name: "apptest2.com"
  }
}

module.exports = (scenario) => {

  test('Category test', async (s, t) => {
    const { liza } = await s.players({liza: one('liza')}, true)

    const app_address = await liza.call("app", "provider", "register_app", App_Config_1);
    t.equal(app_address.Ok.length, 46)
    console.log("APP ADDRESS:: ",app_address.Ok);

    const app_address2 = await liza.call("app", "provider", "register_app", App_Config_2);
    t.equal(app_address2.Ok.length, 46)
    console.log("APP ADDRESS:: ",app_address2.Ok);

    const result1 = await liza.call("app", "categories", "add_category", {category:"Zo", tag:"El", hash:app_address.Ok})
    console.log("add Categories:: ",result1);
    t.equal(result1.Ok, null)

    const result2 = await liza.call("app", "categories", "add_category", {category:"Zo", tag:"El", hash:app_address2.Ok})
    console.log("add Categories:: ",result2);
    t.equal(result2.Ok, null)

    const result3 = await liza.call("app", "categories", "get_apps_by_category", {category:"Zo"})
    console.log("Get Categories:: ",result3);
    console.log("Get Categories:: ",JSON.stringify(result3.Ok[0].entry));
    t.equal(result3.Ok.length, 2)
    await liza.kill()
  })
}
