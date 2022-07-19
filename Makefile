include ./ENV.list
export $(shell sed 's/=.*//' ./ENV.list)

SUBDIRS := \
	cash_service \
	commitment_service \
	customer_service \
	email_service \
	invoice_service \
	latex_service \
	loyalty_service \
	pricing_service \
	procurement_service \
	product_service \
	purchase_service \
	sku_image_service \
	sku_imgprocesser_service \
	source_service \
	stock_service \
	upl_service \
	user_service \
	license \
	api \

all: $(SUBDIRS)

$(SUBDIRS):
	cargo build -p $@ --release
	strip target/release/$@
	cp target/release/$@ ./$@/docker
	docker build -t gardenzillaorg/$@:latest ./$@/docker
	
admin:
	cd admin && ng build --output-path docker/dist
	docker build -t gardenzillaorg/admin ./admin/docker
	docker push gardenzillaorg/admin:latest

store:
	cd store && ng build --output-path docker/dist
	docker build -t gardenzillaorg/store ./store/docker

sync_data:
	echo "Downloading..." \
	&& ssh -p 2279 admin@services.internal.gardenzilla.hu "cd /storage; tar czf - data" > ./data.tar.gz \
	&& echo Data downloaded \
	&& sudo rm -rf data \
	&& tar -xf data.tar.gz \
	&& rm data.tar.gz \
	&& echo Data synced

serve_admin:
	cd admin && ng serve --port 3231

serve_store:
	cd store && ng serve --port 3232
	
docker_start:
	for service in $(SUBDIRS); do \
		docker start gardenzilla_$$service; \
	done

docker_stop:
	for service in $(SUBDIRS); do \
		docker stop gardenzilla_$$service; \
	done

docker_push:
	for service in $(SUBDIRS); do \
		docker push gardenzillaorg/$$service:latest; \
	done

migration:
	cargo build -p migration --release
	strip target/release/$@
	cp target/release/$@ ./bin/$@
  
.PHONY: all $(SUBDIRS) docker_push admin store sync_data serve_admin serve_store docker_start docker_stop migration