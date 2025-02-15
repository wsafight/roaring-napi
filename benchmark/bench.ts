import { Bench } from 'tinybench'
import { RoaringBitmap } from '../index.js'

const b = new Bench()

const bitmap = new RoaringBitmap()

b.add('Rust RoaringBitmap', () => {
  bitmap.add(1)
})

b.add('C++ RoaringBitmap', () => {
  for (let i = 0; i< 1000;i ++) {
    bitmap.add(i)
    bitmap.has(i)
  }
})

await b.run()

console.table(b.table())
