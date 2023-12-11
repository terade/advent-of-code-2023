FMT = black
FILES = *.py
LINT = pylint

run:
	python $(FILES)

fmt: lint
	$(FMT) $(FILES)

lint: 
	$(LINT) $(FILES)
