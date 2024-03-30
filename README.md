# generate-GUI-for-config
A utility for quickly generating HTML settings pages based on configuration files.
## About
This utility need to fast making settings page for configuration. Do not support attachments elements.
## Installation
You may build it from sources or download this from releases page.

For building, you should do:
- Download sources from GitHub page or terminal client.
- Install Rust from [this link](https://www.rust-lang.org/tools/install) or if you use a Linux-like system, then install `cargo` using your package manager.
- If you downloaded the sources from the GitHub page, you need to extract files from archive.
- Go to the directory where you have placed the source code. It must have `Cargo.toml` file in root.
- Open your terminal or console in this directory and run `cargo build --release`.
- After building is done, you can find the executable file at the path `target/release/` you find executable file with name `ggfc`.
## Using
Type this command (on Windows, you need to type `ggfc.exe` instead of `./ggfc`): `./ggfc path/to/your/format -o file/output/name -r path/to/resources`.

There are arguments:

Mandatory:
- `format` - file on the basis of which will be generated settings page.
- `-r` - path to resources needs to generate.

And optional:
- `-o` - which gives output file name.

### Format
A format is a file that contains information about how the settings page should look like.
For example, take bash-script:
```bash
#!\bin\bash
echo 1
echo 2 true
```
This is how the format will look:
```
#!\bin\bash
![space_char  ]
+[echo {}]
[echo('if') {} true|false]
echo @"[some text]@"
```
We see general entry `modifier_symbol(optional)[body]`.
Elements in brackets will be converted to HTML and append to page, but if you need to don't have something `[taxt]` case, then you need to use comments `@"[text]@"`. 
For start comment write `@"` for end same, all that into comments will be ignored. Also, you can use just symbol `@`. After `@` next symbol will be ignored.

#### Modifier symbols
|Modifier symbol|Description|
|---|--------------------------------------------------------------------------------------------------------------------------|
| `+` | means that is an innumerable. In settings, it displays which a list to which you can add unlimited number of elements. |
| `!` | means that is a rule. You can read about it in [rules table](#rules-table). General body empty `[rule value]`.|

#### Rules table
|Rule|Description|
|---|--------------------------------------------------------------------------------------------|
|`space_char`|Sets the character that will be considered as a separator.|

#### Body
The body consists of the following parts:
- `echo` - if it has no properties, the displays as is.
- `{}` - displays in HTML as input field.
- `true|false` - recommended spaceless. It's option Displays as `<select>` tag.

##### Properties
It is indicated in parentheses `()`.

|  Properties      |                                 Description                               |
|------------------|---------------------------------------------------------------------------|
|`'your pseudonym'`|Contains pseudonym of command if you need to rename command. Also, can use `""`|
|`default`         |Options only. Use it if you need to select default value.                    |

### Resources
If you need to change the contents of the resource folder, then follow these recommendations:

`base.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Settings</title>
</head>
<body>
  <!--code-->
  <!--innumerable-->
  <!--numerable-->
  <button onclick="send();">Print</button>
</body>
</html>
```
This is usually an HTML page, but there are comments that will be replaced:

`<!--code-->` - here will be included the contains of scripts.html.

`<!--innumerable-->` - here will be included the contains of innumerable.html and duplicated according to the format number of times.

`<!--numerable-->` - same as previous, but for numerable type and not have an HTML file.

For default `innumerable.html`:
```html
<div class="container"></div>
<button class="add" onclick="addEl($n)">+</button>
```
- Do not erase container class. It needs for finding elements on page.
- Button needs to append elements to list. There is a variable `$n`, it will be replaced by number of innumerable element on HTML building phase.

For the default `scripts.html`:
```html
<script>
  function addEl(i) {
    var fragmentContainer = document.createElement('div');
    fragmentContainer.className = 'html-fragment';
    let hf = '';
    switch (i) {
      //<!--html-fragment-->
    }
    fragmentContainer.innerHTML = hf + '<button onclick="this.parentNode.remove();">✖</button>';
    var div = document.getElementsByClassName('container')[i];
    div.appendChild(fragmentContainer);
  }
  function parse() {
    let innumerable = document.getElementsByClassName('container');
    let numerable = document.getElementsByClassName('numerable');
    let config = '<!--format-->';
    let space_char = '<!--space_char-->';
    const reg = /%\@~\([^)]*\)~\@%/g;
    for (let i = 0; i < innumerable.length; i++) {
      for (children of innumerable[i].children) {
        const matches = config.match(reg);
        config = config.replace(matches, matches + '\n' + matches).replace('%@~(', '').replace(')~@%', '');
        let s = 0;
        for (const child of children.children) {
          let res = '';
          switch (child.tagName.toLowerCase()) {
            case 'label':
              res += child.className;
              break;
            case 'select':
            case 'input':
              res += child.value;
              break;
          }
          config = config.replace('+' + s, res);
          s++;
        }
      }
    }
    config = config.replace(reg, "");
    let s = 0;
    for (let i = 0; i < numerable.length; i++) {
      for (const child of numerable[i].children) {
        let res = '';
        switch (child.tagName.toLowerCase()) {
          case 'label':
            res += child.className;
            res += space_char;
            break;
          case 'select':
          case 'input':
            res += child.value;
            break;
        }
        config = config.replace(s, res); s++;
      }
    }
    return config;
  }
  function send() {
    alert(parse());
  }
</script>
```
- Don't delete functions `parse` and `addEl`.
- Don't rewrite it if you don't know what is work it.

For rewrites:

There is `//<!--html-fragment-->` comments in `addEl` function. It's annotation, which will be replaced by `hf='generates-html-code'`, which is needed to add various numerable elements.
We see `parse` function which is needed to output configuration in original format. There is `<!--format-->` and `<!--space_char-->` in `parse` function. First annotation will be replaced by converted format which is ready to be used. The second annotation will be replaced by the eponymous rule from [rules table](#rules-table).
As for styles, nothing recommendations.
## License
In this project using [MIT license](https://github.com/WDYT32/generate-GUI-for-config?tab=MIT-1-ov-file#)
