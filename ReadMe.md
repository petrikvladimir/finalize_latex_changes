[![Build Status](https://travis-ci.org/voop/finalize_latex_changes.svg?branch=master)](https://travis-ci.org/voop/finalize_latex_changes)

## About
In LaTeX, there is a package called _changes_ which is commonly used to manually track modification in the PDF file. For example to mark that something was added to the file you can use command `\added{new text}`. The compiled PDF will containt colorized text showing which part of the pdf was modified. After you revise the document you can use `[final]` flag in the package to generate not-colorized final version of the PDF. However, the commands remain in the LaTeX source code. They need to be removed in order to start new iteration of changes.

This binary takes given latex source codes and automatically removes artifacts used to mark the chagnes. For example, the aplication transforms the following input:

```
This \added{binary}\deleted{app} is \replaced[id=vp]{perfect}{useless}.
```
into:
```
This binary is perfect.
```

### Warning
The application automatically backup all processing files into the directory `.backup_changes/`. You can change the backup directory by option `-b <DIR>`. However, **do not rely on our backup system and always commit/backup your changes before using this application**.

## Installation

Install using cargo with command
```
cargo install --git https://github.com/voop/finalize_latex_changes.git
export PATH="~/.cargo/bin/:$PATH"
```
Consider appending the previous line (e.g. `export ...`) into your `~/.bashrc` file for the permanent usage of all installed cargo binaries.

Test your installation by
```
finalize_latex_changes -h
```
which should produce something like
```
finalize_latex_changes 0.1.0
Vladimir Petrik <vladko.petrik@gmail.com>
Application parses the LaTeX code and removes artifacts created by "changes" package (e.g. "\added{X}" -> "X").

USAGE:
    finalize_latex_changes [OPTIONS] <INPUT>
...
```

## Quick example

Crate test input:
```sh
mkdir /tmp/flc_test
cd /tmp/flc_test
echo "This \added{binary}\deleted{app} is \replaced[id=vp]{perfect}{useless}." > file.tex
cat file.tex
```
Process file (you can also process directory):
```sh
finalize_latex_changes file.tex
```
Output of the aplication:
```sh
Creating a backup directory: .backup_changes
Processing file.tex
	 backup: .backup_changes/file.tex_1537098847.2559152
	 temporary: file.tex_changes
	 added artifacts   : 1
	 deleted artifacts : 1
	 replaced artifacts: 1
```
```sh
 tree -a
├── .backup_changes
│   └── file.tex_1537099066.1278443
└── file.tex

cat file.tex
This binary is perfect.

cat .backup_changes/file.tex_1537099066.1278443
This \added{binary}\deleted{app} is \replaced[id=vp]{perfect}{useless}.
```
