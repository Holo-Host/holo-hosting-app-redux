const sleep = require('sleep');
module.exports = (scenario) => {
  scenario.runTape('App Flow Test',async (t, {liza}) => {
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
    t.equal(app_address_1.Ok, "QmU2ZBDdD5DSJYJ6wcWwDwVGYtLg4Bc7GhUQoHnp8AC3zu");

    sleep.sleep(5);

    const all_apps = liza.call("host","get_all_apps",{});
    console.log("All Apps: ",all_apps);
    t.equal(all_apps.Ok.length, 2)

})
}
