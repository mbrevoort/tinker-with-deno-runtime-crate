let result = await fetch("http://unallowed.com");
console.log(await result.text()); 