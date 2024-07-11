import two

public func test() throws {
  assert(newRecordDefault() == newRecordDefault())
  assert(newRecordDefault() == newRecord(one: newOneDefault(), two: newTwoDefault()))
  assert(GammaObject.newDefault() == GammaObject.newDefault())
  assert(objectRecord(value: GammaObject.newDefault()) == newRecordDefault())
  assert(recordObject(value: newRecordDefault()) == GammaObject.newDefault())

  let v = complexRecord(
    a: newZeroDefault(),
    b: newOneDefault(),
    c: newRecordDefault(),
    x: ZeroObj.newDefault(),
    y: OneObj.newDefault(),
    z: GammaObject.newDefault()
  )

  complexObject(
    value: v
  )

}

try! test()
