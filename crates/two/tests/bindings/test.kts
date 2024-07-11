import com.sajjon.zero.*
import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    assert(newRecordDefault() == newRecordDefault())
    assert(newRecordDefault() == newRecord(one = newOneDefault(), two = newTwoDefault()))
    assert(GammaObject.newDefault() == GammaObject.newDefault())
    assert(objectRecord(value = GammaObject.newDefault()) == newRecordDefault())
    assert(recordObject(value = newRecordDefault()) == GammaObject.newDefault())
  
    val r = newRecord(one = newOne(value = newZero(value = true)), two = newTwo(value = true))
    assert(r == recordRecord(value = r))
    assert(r == recordRefRecord(value = r))
  
    val o = GammaObject(one = newOne(value = newZero(value = true)), two = newTwo(value = true))
    assert(o == objectObject(value = o))
    assert(o == objectRefObject(value = o))
}

test()