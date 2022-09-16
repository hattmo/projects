function(context, args) {
	var r = #fs.scripts.fullsec()
	var n = #fs.users.last_action({ name: "gibson"})[0].t.getTime()
	var g = []
	var i = #db.f({ _id: "f" }).first().i
	if (i + 5 > r.length) i = 0
	var e = i + 5
	while (i < e) {
		#ms.chats.join({name: r[i]})
		var f = #fs.scripts.fullsec({sector: r[i]})
		#ms.chats.leave({channel: r[i]})
		for (var j = 0; j < f.length; j++) {
			if(#fs.users.last_action({ name: f[j].split(".")[0] })[0].t.getTime() === n) g.push(f[j])
		}
		i++
	}
	#db.u({ _id: "f" },{$set:{"i":e}})
	#db.u({ _id: "f" },{$addToSet:{"v":{$each:g}}})
	return g
}