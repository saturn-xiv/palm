# Usage

$()$(
    bash
    $ podman run --rm -it --events-backend=file --network host -v $PWD:/workspace:z ubuntu:jammy
    /workspace >cd
    >./build.sh
    -lh >ls build/Release/loquat
)$()
