import * as rs from '../rs/src/lib.rs';

console.log(`3 + 2 = ${rs.add(3, 2)}`);
console.log(rs.memory);

const arr = new Int32Array(rs.memory.buffer, 0, 5);

arr.set([1, 2, 3, 4, 5]);

console.log(`sum([1, 2, 3, 4, 5]) = ${rs.sum(0, 5)}`);

rs.inc(0, 5);
console.log(`inc([1, 2, 3, 4, 5]) = ${arr.slice(0, 5)}`);

console.log(rs.memory);