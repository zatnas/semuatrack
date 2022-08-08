console.log("YEEHAW");
fetch("/api/transaction")
	.then((response) => response.json())
	.then((data) => {
		console.log(data);
	});
