const {init} = require('../index');
console.time("cost：")
console.log(init({path:'./test',alias:{"@":"./src"}}));
console.timeEnd("cost：")