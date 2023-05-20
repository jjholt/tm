# TeX Manager
This is a manager for your LaTeX projects. it's intended to provide a quick and easy way to do repetitive organisational tasks.

One example is organisation of large chapters, which requires creating a file `my_chapter.tex` in `src`, then a folder of the same name, and finally it needs to be imported in the index with `\subimport{src/}{my_chapter}`.
With tm, simply use `tm ac "My Chapter"`.
