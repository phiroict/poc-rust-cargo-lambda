init:
	cargo install cargo-lambda
	cargo install cross
build:
	cargo lambda build --release
build_container:
	cargo lambda build --compiler cross --release
deploy:
	aws-vault exec home --backend=kwallet  --region ap-southeast-2 --no-session --  cargo lambda deploy
local:
	cargo lambda watch