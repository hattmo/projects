function(context, args) {
    var l = args.l
    var o = #db.f({ _id: "l" }).first()
    var t = " "
    var a = {}
    var r = l.call(a);
    do {
        var m = r.match(/`N(\w+)`.*$/)
        if (m) {
            t = m[1]
            var i = 0
        }
        if (t[0] == "E") a[t] = o.v[i++]
        if (t[0] == "d") a[t] = i++
        if (t[0] == "e") a[t] = o.p[i++]
        if (t[0] == "c") {
            a[t] = o.c[i++]
            if (t[3] == "1") a.color_digit = a[t].length
            if (t[3] == "2") a.c002_complement = o.c[(i + 3) % 8]
            if (t[3] == "3") {
                a.c003_triad_1 = o.c[(i + 6) % 8]
                a.c003_triad_2 = o.c[i]
            }
        }
        if (t[0] == "l") a[t] = o.l[i++]
        r = l.call(a);
    } while (!r.match(/terminated/));
    return a
}