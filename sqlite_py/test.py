import sqlite3

con = sqlite3.connect("test.db")
cur = con.cursor()
res = cur.execute("SELECT score FROM movie")
print(res.fetchall())
sqlite3.SQLITE_DROP_TABLE