let result = await fetch("http://dinoipsum.herokuapp.com/api/?format=text&paragraphs=1&words=3");
console.log(await result.text()); 