const {init} = require('../index');
console.time("cost：")
console.log(init({path:'./test',alias:{"@":"./src"},npmPackages:[], excludes:["**/*.tsx"]},));
console.timeEnd("cost：")