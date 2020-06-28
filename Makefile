.PHONY: build_venv build_dir build_config build_rust build_scripts build_vizualizer build_simumap \
		clean


# setup to make a build
build: build_dir build_venv build_scripts build_config build_rust build_vizualizer build_simumap

build_rust: build_dir
	@echo "Building rust simulator"
	cargo build --release --manifest-path="simulator/Cargo.toml"
	cp -r simulator/target/release build
	if [ -d build/simulator ]; then rm -r build/simulator; fi
	mv build/release build/simulator

build_venv: build_dir
	if [ ! -d build/venv ]; then virtualenv build/venv --no-site-packages; fi

build_simumap: build_dir build_venv
	@echo "Building Simumap"
	cp -r simumap build
ifeq ($(OS),Windows_NT)
	scripts/pip_install.sh build/venv/Scripts/activate -r simumap/requirements.txt
	scripts/pip_install.sh build/venv/Scripts/activate ./simumap
	echo "export PYTHONPATH='${PYTHONPATH};${CURDIR}/build/simumap/simumap'" >> build/venv/Scripts/activate
else
	scripts/pip_install.sh build/venv/bin/activate -r simumap/requirements.txt
	scripts/pip_install.sh build/venv/bin/activate ./simumap
	echo "export PYTHONPATH='$(PYTHONPATH):`pwd`/simumap/simumap'" >> build/venv/bin/activate
endif

build_vizualizer: build_dir
	@echo "Building Visualizer"
ifeq ($(OS),Windows_NT)
		scripts/pip_install.sh build/venv/Scripts/activate -r ./visualizer/requirements.txt
else
		scripts/pip_install.sh build/venv/bin/activate -r ./visualizer/requirements.txt
endif
	cp -r visualizer build

build_scripts: build_dir
	cp -r scripts/simumo.sh build
	cp -r scripts/simumo_viz.sh build
	cp -r scripts/simumo_sim.sh build

build_config: build_dir
	cp -r simulator/etc build

build_dir:
	mkdir -p build


# setup for a proper dev environment
dev : dev_venv

dev_venv:
	@echo "creating dev environment"
	if [ ! -d venv ]; then virtualenv venv --no-site-packages; fi

ifeq ($(OS),Windows_NT)
		scripts/pip_install.sh venv/Scripts/activate -r simumap/requirements.txt
		scripts/pip_install.sh venv/Scripts/activate -r visualizer/requirements.txt
		scripts/pip_install.sh venv/Scripts/activate ./simumap
		echo "export PYTHONPATH='${PYTHONPATH};${CURDIR}/simumap/simumap'" >> venv/Scripts/activate
else
		scripts/pip_install.sh venv/bin/activate -r simumap/requirements.txt
		scripts/pip_install.sh venv/bin/activate -r visualizer/requirements.txt
		scripts/pip_install.sh venv/bin/activate ./simumap
		echo "export PYTHONPATH='$(PYTHONPATH):`pwd`/simumap/simumap'" >> venv/bin/activate
endif

clean: dev_clean build_clean

dev_clean:
	@echo "Cleaning up dev"
	if [ -d venv ]; then  rm -r venv; fi;

build_clean:
	@echo "Cleaning up build"
	if [ -d build ]; then rm -r build; fi;

