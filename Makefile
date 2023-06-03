make:
	rustc js.rs
	gzip jscli.1 -k
