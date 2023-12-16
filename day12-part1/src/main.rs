#![feature(test)]
extern crate test;

use std::fs;

use cached::UnboundCache;

#[macro_use] extern crate cached;


#[derive(Hash, PartialEq, Eq)]
struct Case {
    line: String,
    spans: Vec<usize>, 
    cur_span: Option<usize>
}

cached_key! {
    COUNT_POSSIBILITIES: UnboundCache<Case, usize> = UnboundCache::new();
    Key = {
        Case{line: line.into(), spans: spans.into(), cur_span}
    };
    fn count_possibilities(line: &str, spans: &[usize], cur_span: Option<usize>) -> usize = {
        if line.is_empty() {
            if spans.is_empty() {
                if cur_span.is_some() {
                    return 0
                } else {
                    return 1
                }
            } else if spans.len() == 1 && cur_span.is_some_and(|span_length| span_length == spans[0]) {
                return 1;       
            } else {
                return 0;
            }
        }
    
        if let Some(span_length) = cur_span {
            if line.as_bytes()[0] == b'#' {
                return count_possibilities(&line[1..line.len()], spans, Some(span_length + 1));
            } else if line.as_bytes()[0] == b'.' {
                if spans.is_empty() {
                    return 0;
                } else if span_length == spans[0] {
                    return count_possibilities(&line[1..line.len()], &spans[1..spans.len()], None)
                } else {
                    return 0;
                }
            } else if line.as_bytes()[0] == b'?' {
                let dot_case = {
                    if spans.is_empty() {
                        0
                    } else if span_length == spans[0] {
                        count_possibilities(&line[1..line.len()], &spans[1..spans.len()], None)
                    } else {
                        0
                    }
                };
    
                let hash_case = count_possibilities(&line[1..line.len()], spans, Some(span_length + 1));
    
                return dot_case + hash_case;
            }
        } else { // Not in span
            if line.as_bytes()[0] == b'#' {
                // Start new span
                return count_possibilities(&line[1..line.len()], spans, Some(1));
            } else if line.as_bytes()[0] == b'.' {
                return count_possibilities(&line[1..line.len()], spans, None);
            } else {
                return count_possibilities(&line[1..line.len()], spans, Some(1)) + count_possibilities(&line[1..line.len()], spans, None);
            }
        }

        println!("{} {:?} {:?}", line, spans, cur_span);
    
        unreachable!()
    }    
}

fn do_aoc(input: &str) -> usize {
    input.lines().map(|line| {
        let (line, spans_str) = line.split_once(' ').unwrap();
        let spans: Vec<usize> = spans_str.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        
        count_possibilities(line, &spans, None)
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

#[bench]
fn bench_day12(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
