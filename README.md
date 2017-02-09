# Rust Terminal Colourizer

Takes input, colourizes it.

## Caveats

*WIP: Does not yet properly colour segments of lines, only whole lines.*

Takes arguments equal to number of total capture groups + 1, but currently only colourizes the whole line.

## Usage

    ./rust-term-colourizer regex_file

By default, reads from STDIN. Pipe in input to colourize and exit.

## Regex file

Regex contain a series of expressions, differentiated by the first character:

- `/<regex>`: Regular expression, e.g. `/^test$`.
- `+<b|u|bu>`: Define a style (bold/underline/both). Must follow regular expression, e.g. `+bu`.
- `=<colour>`: Define a colour, e.g. `=red`.
    - Supported colours: `black`, `red`, `green`, `yellow`, `blue`, `purple`, `cyan`, `white`.
    - Special option: `default`, used if just bold/underline are desired.
    - Must follow style definitions, or regular expression if no styles are defined.
- `-`: Do not print this line. Must follow regular expression.
- `#<text>`: A comment, ignored when parsing.

### Sample regex file

Here's a sample file that can be used to colourize Org-mode agenda output:

```
# Comment

/^Global list
+bu
=white

/^Week-agenda
+bu
=white

/^[A-Za-z]+day
+b
=green

/TODO|NEXT|IN-PROGRESS
=cyan

/:quiz:|:midterm:|:final:
=red

/:review:
=yellow

/==========
-
```

### Sample output

![Sample output][img/screenshot.png]
