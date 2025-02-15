const { RoaringBitmap } = require('./index')

console.log(RoaringBitmap)

const bitmap = new RoaringBitmap()

// 插入元素
bitmap.add(1)
bitmap.add(2)
bitmap.add(3)
bitmap.add(100000000)

// 检查元素是否存在
console.log('Contains 2:', bitmap.has(2))

// 移除元素
bitmap.remove(2)
console.log('Contains 2 after removal:', bitmap.has(2))

// 获取元素数量
console.log('Bitmap length:', bitmap.len())

// 判断是否为空
console.log('Is bitmap empty:', bitmap.isEmpty())

const bitmap2 = new RoaringBitmap()

bitmap2.add(1)
bitmap2.add(1000)
bitmap2.add(10)

const bb =bitmap2.serialize()
const buf = new Uint8Array(bb);



// console.log(buf.toLocaleString())

console.log('计算',
  bitmap2.serialize().toString() == [...buf].toString(),
)

const bitmap3 = new RoaringBitmap()

const arr = [58, 48, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 16, 0, 0, 0, 1, 0, 10, 0, 232, 3]

// bitmap3.deserialize(arr)

console.log(bitmap3.len())

console.log(bitmap3.has(1000))

const bitmap4 = new RoaringBitmap()

bitmap4.deserialize(arr)

console.log(bitmap4.len())
