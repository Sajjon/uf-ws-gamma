import two

public func test() throws {

  let record = GammaRecord(
    zeroRecord: ZeroRecord(value: false),
    zeroObject: ZeroObject(value: ZeroRecord(value: false)),
    oneRecord: OneRecord(
      zeroRecord: ZeroRecord(value: false),
      zeroObject: ZeroObject(value: ZeroRecord(value: false))
    ),
    oneObject: OneObject(
      one: newOneDefault(),
      zeroRecord: ZeroRecord(value: false),
      zeroObject: ZeroObject(value: ZeroRecord(value: false))
    ),
    two: newTwo(value: false)
  )
  let object = recordToObject(record: record)
  assert(object == GammaObject.newDefault())
}

try! test()
