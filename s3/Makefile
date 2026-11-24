.PHONY: test

test:
	cargo test --locked -p s3


.PHONY: integration-test
integration-test:
	@echo "Running S3 integration tests against an already-running MinIO instance"
	@echo "Tip: run MinIO yourself (example image: minio/minio) before this target"
	S3_RUN_INTEGRATION=1 \
	S3_ENDPOINT=$${S3_ENDPOINT:-http://127.0.0.1:9000} \
	S3_REGION=$${S3_REGION:-auto} \
	S3_ACCESS_KEY_ID=$${S3_ACCESS_KEY_ID:-minioadmin} \
	S3_SECRET_ACCESS_KEY=$${S3_SECRET_ACCESS_KEY:-minioadmin} \
	S3_TEST_BUCKET=$${S3_TEST_BUCKET:-rust-stdx-s3-integration} \
	cargo test --locked -p s3 --test minio_integration -- --nocapture  --test-threads=1
