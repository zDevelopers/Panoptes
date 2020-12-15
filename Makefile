install:
	cd front && npm install

run-front:
	cd front && npm run serve

run-back:
	cd back && cargo run

run:
	make -j2 run-back run-front

extract-translations:
	python scripts/extract_translations.py
