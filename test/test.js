const {init} = require('../index');
console.time("cost：")
console.log(init({path:'./test',alias:{"@":"./src"},npmPackages:["koa-router"], excludes:["**/*.tsx"]},));
console.timeEnd("cost：")