function(context, args) {
    if (args.a) {
        if (args.a === "check") {
            return #db.f({ _id: "f" }).array()
        } else if (args.a === "reset") {
            #db.u({ _id: "f" }, { $set: { i: 0, v: [] } })
            return #db.f({ _id: "f" }).array()
        }
    }
}