//	{{{3
//	vim: set tabstop=4 modeline modelines=10:
//	vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//	{{{2
#include <vector>
#include <cstdio>
using namespace std;
//	Ongoings:
//	{{{
//	Ongoing: 2023-02-09T21:15:24AEDT Linux requires '-fno-strict-aliasing' to build(?)
//	Ongoing: 2023-02-09T21:17:23AEDT contention: UB pointer aliasing examples do not fail (UB checking tools) AddressSanitizer / UBSan
//	}}}

//	Unsafe Rust is not C (C++ examples):
//  LINK: https://www.youtube.com/watch?v=DG-VLezRkYQ

void first()
{
	//	UB: pointer is invalidated when we call 'push_back'
	vector<int> v = {1,2,3,4};
	const int* pv = &v[0];
	v.push_back(5);
	printf("%d\n", *pv);
}


int foo_i(int& x, int& y) {
	x = 42;
	y = 99;
	return x;
}
int foo_ii(int* x, int* y) {
	*x = 42;
	*y = 99;
	return *x;
}
//	'__restrict__' ('restrict' in C) tells the compiler pointers do not overlap
//	(this can allow significant opimizations)
int foo_iii(int* __restrict__ x, int* __restrict__ y) {
	*x = 42;
	*y = 99;
	return *x;
}
void second()
{
	int n;
	n = 0;
	printf("%d\n", foo_i(n, n));
	n = 0;
	printf("%d\n", foo_ii(&n, &n));

	//	UB: 'restrict' keyword requires pointers not alias
	n = 0;
	printf("%d\n", foo_iii(&n, &n));		
}


//	Strict aliasing rule: Compiler is permitted to assume pointers of different types do not alias
//	(Accessing 'Bar' though a 'Foo' pointer is also UB?)
typedef struct Foo { int x; } Foo;
typedef struct Bar { int x; } Bar;
int f(Foo* foo, Bar* bar) {
	foo->x = 42;
	bar->x = 99;
	return foo->x;
}
void pointer_aliasing_different_pointer_types()
{
	//	UB: outputs 42/99 depending on optimization level
	//	(correct when given '-fno-strict-aliasing' flag)
	struct Foo foo;
	int output = f(&foo, (Bar*) &foo);
	printf("output=(%d)\n", output);
}


//	(partial overlapping is UB)
//	Pointers to different objects must either be the same, or not overlap at all
typedef struct Baz { int x; int y; } Baz;
int g(Baz* b1, Baz* b2) {
	b1->y = 42;
	b2->x = 99;
	return b1->y;
}
void pointer_aliasing_partial_overlapping()
{
	//	UB: outputs 42/99 depending on optimization level
	//	(correct when given '-fno-strict-aliasing' flag)
	int a[3] = {0};
	Baz* b1 = (Baz*) &a[0];
	Baz* b2 = (Baz*) &a[1];
	int output = g(b1, b2);
	printf("output=(%d)\n", output);
}


int main()
{
	first();
	second();
	pointer_aliasing_different_pointer_types();
	pointer_aliasing_partial_overlapping();
	return 0;
}

