dev: stop
	yarn install
	yarn tauri dev 

stop:
	killall node npm | true

TAG=$(shell git tag -l "*tauriWallet*")
clean-tags:
	git push origin --delete ${TAG}
	git tag -d ${TAG}