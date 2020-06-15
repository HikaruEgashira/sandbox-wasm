import module from "../crate/Cargo.toml";
module.run();

const arr = (module.arr() as Array<number>).map((ar) => ar * 2);
console.log(arr);

// console.log(module.ls()); panic
