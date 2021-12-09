use std::{
    io::{self, BufRead, BufReader},
    collections::{BTreeSet, HashMap}
};

use itertools::Itertools;

pub fn simple()
{
    let mut count = 0;
    for line in BufReader::new(io::stdin()).lines().map(|line| line.unwrap()) {
        count += line
            .split('|').nth(1).unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|string| string.as_bytes().len())
            .filter(|&x| x == 2 || x == 3 || x == 4 || x == 7)
            .count();
    }

    println!("{}", count);
}

//
//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
//

// 0 - 6 segs
// 1 - 2 segs
// 2 - 5 segs
// 3 - 5 segs
// 4 - 4 segs
// 5 - 5 segs
// 6 - 6 segs
// 7 - 3 segs
// 8 - 7 segs
// 9 - 6 segs

// 1.  1, 4, 7, 8 are unique
// 2.  by 7 - 1 we can deduce aaaa (unneeded)
// 3.  by 1 - 6segs we can deduce cc
// 4.  by 1 we can deduce ff
// 5.  3 is the only 5seg that has both cc and ff on
// 6.  2 is the only 5seg that has cc on but not ff
// 7.  5 is the only 5seg that has ff on but not cc
// 8.  6 is the only 6seg that has ff on but not cc
// 9.  add cc to 5 - we have 9
// 10. 0 is the last

type SevenSeg = BTreeSet<u8>;
type Digits = Vec<SevenSeg>;
type Mapping<'a> = [Option<&'a SevenSeg>; 10];
type ReverseMapping<'a> = HashMap<&'a BTreeSet<u8>, i32>;

// This could be more compact but let's try to do it "the scripting language way", procedurally, no functions/structs
pub fn complex()
{
    let mut sum = 0;
    for line in BufReader::new(io::stdin()).lines().map(|line| line.unwrap()) {
        let mut mapping: Mapping = Default::default();
        let mut rev_mapping: ReverseMapping = ReverseMapping::new();

        let (inputstr, outputstr) = line.split('|').tuples().next().unwrap();
        let inputs: Digits = inputstr.split(' ')
            .filter(|x| !x.is_empty())
            .map(|digitstr| digitstr.bytes().collect())
            .collect();
        let outputs: Digits = outputstr.split(' ')
            .filter(|x| !x.is_empty())
            .map(|digitstr| digitstr.bytes().collect())
            .collect();

        // 1st step
        let one   = inputs.iter().find(|x| x.len() == 2).unwrap();
        let four  = inputs.iter().find(|x| x.len() == 4).unwrap();
        let seven = inputs.iter().find(|x| x.len() == 3).unwrap();
        let eight = inputs.iter().find(|x| x.len() == 7).unwrap();

        rev_mapping.insert(one, 1);
        rev_mapping.insert(four, 4);
        rev_mapping.insert(seven, 7);
        rev_mapping.insert(eight, 8);
        mapping[1] = Some(one);
        mapping[4] = Some(four);
        mapping[7] = Some(seven);
        mapping[8] = Some(eight);

        // 2nd step
        // let aaaa = *mapping[7].difference(&mapping[1]).next().unwrap();

        // 3rd step
        let six_segs: Digits = inputs.iter()
            .filter(|x| x.len() == 6)
            .cloned()
            .collect();

        let cc = *mapping[1].unwrap().iter().find(|seg| six_segs.iter().any(|digit| !digit.contains(seg))).unwrap();

        // 4th step
        let ff = *mapping[1].unwrap().iter().find(|&&seg| seg != cc).unwrap();

        // 5th step
        let three = inputs.iter()
            .filter(|x| x.len() == 5)
            .find(|x| x.contains(&cc) && x.contains(&ff))
            .unwrap();

        // 6th step
        let two = inputs.iter()
            .filter(|x| x.len() == 5)
            .find(|x| x.contains(&cc) && !x.contains(&ff))
            .unwrap();

        // 7th step
        let five = inputs.iter()
            .filter(|x| x.len() == 5)
            .find(|x| !x.contains(&cc) && x.contains(&ff))
            .unwrap();

        // 8th step
        let six = inputs.iter()
            .filter(|x| x.len() == 6)
            .find(|x| !x.contains(&cc) && x.contains(&ff))
            .unwrap();

        rev_mapping.insert(three, 3);
        rev_mapping.insert(two, 2);
        rev_mapping.insert(five, 5);
        rev_mapping.insert(six, 6);
        mapping[3] = Some(three);
        mapping[2] = Some(two);
        mapping[5] = Some(five);
        mapping[6] = Some(six);

        // 9th step
        let mut nine = five.clone();
        nine.insert(cc);

        rev_mapping.insert(&nine, 9);
        mapping[9] = Some(&nine);

        // 10th step
        let zero = inputs.iter().filter(|x| !rev_mapping.keys().contains(x)).next().unwrap();

        rev_mapping.insert(&zero, 0);
        mapping[0] = Some(&zero);

        sum +=
            rev_mapping[&outputs[0]] * 1000 +
            rev_mapping[&outputs[1]] * 100 +
            rev_mapping[&outputs[2]] * 10 +
            rev_mapping[&outputs[3]];
    }

    println!("{}", sum);
}
