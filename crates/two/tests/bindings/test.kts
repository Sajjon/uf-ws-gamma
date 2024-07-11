import com.sajjon.zero.*
import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    val record = GammaRecord(
        zeroRecord = ZeroRecord(value = false),
        zeroObject = ZeroObject(value = ZeroRecord(value = false)),
        oneRecord = OneRecord(
            zeroRecord = ZeroRecord(value = false),
            zeroObject = ZeroObject(value = ZeroRecord(value = false))
        ),
        oneObject = OneObject(
            one = newOneDefault(),
            zeroRecord = ZeroRecord(value = false),
            zeroObject = ZeroObject(value = ZeroRecord(value = false))
        ),
        two = newTwo(value = false)
    )

    val gammaObject = recordToObject(record = record)
    assert(gammaObject == GammaObject.newDefault())
}

test()