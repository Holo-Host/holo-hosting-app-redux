const test = require('tape');
const sleep = require('sleep');

module.exports = (liza) => {
  test('App Flow', (t) => {
    const App_Config_1 = {
      ui_hash: "Quarnnnnvltuenb###CONFIG1",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    const App_Config_2 = {
      ui_hash: "Quarnnnnvltuenb###CONFIG2",
      dna_list: ["QweAFioina","QtavsFdvva"]
    }
    t.plan(4)
    const app_address_1 = liza.call("provider", "main", "register_app", App_Config_1);
    const app_address_2 = liza.call("provider", "main", "register_app", App_Config_2);
    console.log("APP ADDRESS:: ",app_address_1);
    t.equal(app_address_1.Ok.length, 46)

    const app_enable = liza.call("host","main","enable_app",{app_hash:app_address_1.Ok});
    liza.call("host","main","enable_app",{app_hash:app_address_2.Ok});
    t.equal(app_enable.Ok, null)

    sleep.sleep(5);

    const app_list = liza.call("host","main","get_enabled_app",{});
    console.log("APP List:: ",app_list);
    t.equal(app_list.Ok.length, 2)

    const agent_list = liza.call("host","main","get_host_for_app",{app_hash:app_address_2.Ok});
    console.log("Agent List:: ",agent_list);
    t.equal(agent_list.Ok.length, 1)

  })
}
