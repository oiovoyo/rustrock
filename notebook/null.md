# null

在 “Null References: The Billion Dollar Mistake” 中，Tony Hoare，null 的发明者，曾经说到：

	`I call it my billion-dollar mistake. At that time, 

    I was designing the first comprehensive type system for 
	
	references in an object-oriented language. 
	
	My goal was to ensure that all use of references should be absolutely safe, 
	
	with checking performed automatically by the compiler. 
	
	But I couldn't resist the temptation to put in a null reference, 
	
	simply because it was so easy to implement. 
	
	This has led to innumerable errors, vulnerabilities, 
	
	and system crashes, 
	
	which have probably caused a billion dollars of pain and damage in the last forty years.`

> 所以rust没有null

> 代之以Option宏，来表达空置

> Option被包含在prelode，不需要任何引用就可以调用

> 当使用Option时，在使用具体值时转换，如果为空转换将失败

> 以此来确保值不为空

> 
