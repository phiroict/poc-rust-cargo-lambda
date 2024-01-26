init:
	cargo install cargo-lambda
build:
	cargo lambda build --release
deploy:
	aws-vault exec home --backend=kwallet  --region ap-southeast-2 --no-session --  cargo lambda deploy
local:
	cargo lambda watch