import crate from './crate';

const i32arr = crate.alloc['i32[]'](15);
const f32arr = crate.alloc['f32[]'](4);

console.log(`allocated [${i32arr}]`);
console.log(`allocated [${f32arr}]`);