#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");
    for (name, input) in [
        ("rfc3339_nano", "2021-11-08T00:54:37.059879000Z"),
        ("rfc2822", "Mon, 08 Nov 2021 00:35:18 +0000"),
        ("yyyy-mm-dd hh:mm:ss", "2012-08-03 18:31:59.257000000"),
        (
            "yyyy-mm-dd hh:mm:ss z",
            "2012-08-03 18:31:59.257000000 +0000",
        ),
        ("yyyy-mm-dd", "2021-02-21"),
        ("yyyy-mon-dd", "2021-Feb-21"),
        ("Mon dd, yyyy, hh:mm:ss", "May 8, 2009 5:57:51 PM"),
        ("dd Mon yyyy hh:mm:ss", "14 May 2019 19:11:40.164"),
        ("dd Mon yyyy", "03 February 2013"),
        ("mm/dd/yyyy hh:mm:ss", "8/8/1965 01:00:01 PM"),
        ("mm/dd/yyyy", "03/31/2014"),
        ("yyyy/mm/dd hh:mm:ss", "2012/03/19 10:11:59"),
        ("yyyy/mm/dd", "2014/03/31"),
        ("mm.dd.yyyy", "03.31.2014"),
        ("yyyy.mm.dd", "2014.03.30"),
        ("yymmdd hh:mm:ss", "171113 14:14:20"),
        ("chinese yyyy mm dd hh mm ss", "2014年04月08日11时25分18秒"),
        ("chinese yyyy mm dd", "2014年04月08日"),
        ("timezone_abbrev", "2017-11-25 13:31:15 PST"),
    ]
    .iter()
    {
        group.bench_function(*name, |b| {
            b.iter(|| {
                let _res = anydate::parse_utc(input);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
