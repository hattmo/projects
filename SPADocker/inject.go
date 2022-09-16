package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"

	"golang.org/x/net/html"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func createInject() {
	inject, err := os.Create("/usr/share/nginx/html/inject.js")
	check(err)
	defer inject.Close()

	inject.WriteString("window.process = {};\n")
	inject.WriteString("window.process.env = {};\n")
	for _, e := range os.Environ() {
		result := strings.Replace(e, "\\", "\\\\", -1)
		pair := strings.SplitN(result, "=", 2)
		inject.WriteString("window.process.env[\"" + pair[0] + "\"] = \"" + pair[1] + "\";\n")
	}
}

func appendIndex() {
	index, e := os.Open("/usr/share/nginx/html/index.html")
	check(e)
	nodes, e := html.Parse(index)
	check(e)
	index.Close()
	scriptNode := &html.Node{Type: html.ElementNode, Data: "script", Attr: []html.Attribute{html.Attribute{Key: "src", Val: "/inject.js"}}}
	rootTag := nodes.FirstChild
	searchTag := rootTag.FirstChild

	for searchTag.Data != "head" {
		if searchTag.Type == html.ErrorNode {
			emptyHeadTag := &html.Node{Type: html.ElementNode, Data: "head"}
			rootTag.AppendChild(emptyHeadTag)
			searchTag = emptyHeadTag
		} else {
			searchTag = searchTag.NextSibling
		}
	}
	searchTag.AppendChild(scriptNode)
	index, e = os.Create("/usr/share/nginx/html/index.html")
	check(e)
	html.Render(index, nodes.FirstChild)
	index.Close()
}
func editPort() {
	port := "80"
	for _, e := range os.Environ() {
		pair := strings.SplitN(e, "=", 2)
		if pair[0] == "PORT" {
			port = pair[1]
		}
	}

	config, e := ioutil.ReadFile("/etc/nginx/conf.d/default.conf")
	check(e)
	newConf := strings.Replace(string(config), "${PORT}", port, 1)
	out, e := os.Create("/etc/nginx/conf.d/default.conf")
	check(e)
	out.WriteString(newConf)
	out.Close()
}

func main() {
	fmt.Print("Injecting Environment Vars\n")
	createInject()
	appendIndex()
	editPort()
}
