const { RoaringBitmap } = require('./index')

console.log(RoaringBitmap)

const bitmap = new RoaringBitmap();

// 插入元素
bitmap.insert(1);
bitmap.insert(2);
bitmap.insert(3);
bitmap.insert(100000000)

// 检查元素是否存在
console.log('Contains 2:', bitmap.contains(2));

// 移除元素
bitmap.remove(2);
console.log('Contains 2 after removal:', bitmap.contains(2));

// 获取元素数量
console.log('Bitmap length:', bitmap.len());

// 判断是否为空
console.log('Is bitmap empty:', bitmap.isEmpty());
