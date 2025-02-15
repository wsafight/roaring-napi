import test from 'ava'
import { RoaringBitmap } from '../index'

test('sync function from native code', (t) => {
  const current = new RoaringBitmap();
  t.is(current.len(), 0)
})
