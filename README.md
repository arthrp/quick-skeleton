# quick-skeleton

[![Build Status](https://travis-ci.org/arthrp/quick-skeleton.svg?branch=master)](https://travis-ci.org/arthrp/quick-skeleton)

**tldr;** Lightweight replacement for [yeoman](http://yeoman.io/) or [slush](http://slushjs.github.io). Powered by [handlebars](https://github.com/wycats/handlebars.js).

This is a scaffolding tool to save you hours of writing boilerplate code for your new project. Many langugages have tools that help you create a project skeleton in a matter of seconds (e.g. in Rust you can use ```cargo``` for that), but this tool isn't tied to any particular language or ecosystem. Just like Yeoman, you can use it for anything. Unlike Yeoman, it tries to be totally simple and does only one thing: replaces Handlebars expressions with values that you provide.

### Template structure

Template is essentially a zip archive that must contain a file called **parameters.json**. That file contains a JSON array with all the expressions that will be replaced with user-provided values. The format is as follows:
```
[{
  "name" : "example", //This is the name of expression
  "value": "", //Default value (currently always overwritten by user's input)
  "desc": "Provide example value" //Text that will be shown to user when asking for value
  }
]
```
All the other files in archive will be extracted and expressions inside them will be processed by Handlebars engine.

### Usage

```quick-skeleton -c [path to template]``` You can use simple_page.zip in project's root to create, well, simple web page. ```quick-skeleton -c simple_page.zip```
