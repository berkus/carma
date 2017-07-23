# Roadkill

This is a viewer for Carmageddon resource files and models.

    $ brew install glfw glm
    $ mkdir _build_
    $ cd _build_
    $ cmake -G Ninja ..
    $ ninja

This will generate several executables, use `glook` to load a model.

    $ glook DecodedData/DATA/64X48X8/CARS/SCREWIE.ENC
