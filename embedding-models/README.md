# embedding-models

Local models (ONNX files, tokenizer config, etc.) used for article vectorization (embedding) live here.

These files are large binaries, so they are not committed to the repository (everything except `README.md` is excluded via `.gitignore`). Download and place them yourself.

Expected directory layout:

```
embedding-models/
  README.md
  <org-name>/
    <model-name>/
      model.onnx
      tokenizer.json
      config.json
```
