#![feature(test)]
#![feature(step_by)]

extern crate test;

fn for_loop()
{
	for _ in (0..1_000_000).step_by(test::black_box(1))
	{
	}
}

fn while_loop()
{
	let mut i=0;
	while i<1_000_000
	{
		i+=test::black_box(1);
	}
}

#[bench]
fn bench_for(bench:&mut test::Bencher)
{
	bench.iter(for_loop);
}

#[bench]
fn bench_while(bench:&mut test::Bencher)
{
	bench.iter(while_loop);
}

