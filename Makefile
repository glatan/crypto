CONTAINER_NAME = libre_crypto

.PHONY: p.build
p.build:
	@podman build -t ${CONTAINER_NAME} .

.PHONY: run.bash
run.bash:
	-@podman run --name $@ -v .:/workdir -it ${CONTAINER_NAME} bash
	@podman rm $@
