.PHONY:
dev: docker-compose-up create-s3-buckets run-migrations
	# done;

.PHONY:
docker-compose-up:
	# running docker-compose up;
	docker-compose up -d;
	# giving time for docker-compose containers to finish starting;
	sleep 15;

.PHONY:
run-migrations:
	# running migrations
	sqlx migrate run

.PHONY:
create-s3-buckets:
	# --endpoint-url is the s3 endpoint defined in the docker-compose file.
	# creating s3 buckets;
	aws --endpoint-url=http://127.0.0.1:4566 s3api create-bucket --bucket local-betarme-user-content;
	# aws --endpoint-url=http://127.0.0.1:4566 s3api list-objects --bucket local-betarme-user-content
	# aws --endpoint-url=http://127.0.0.1:4566 s3api list-buckets