use rust_htslib::bcf::Read;
use rust_htslib::bcf::Reader;
use std::convert::TryFrom;
use std::iter::repeat_with;

struct Alleles {
    sref: i64,
    salt: i64,
    snmiss: i64,
}

impl Alleles {
    fn new() -> Alleles {
        Alleles {sref: 0, salt: 0, snmiss: 0}
    }
}

pub fn parse_vcf(path: &str){
    let mut byvector = false ;
    let mut bcf = rust_htslib::bcf::Reader::from_path(path).expect("Error opening file.");
    //get sample count
    let mut sample_count:usize = 0;
    for (i, record_result) in bcf.records().enumerate(){
        let record = record_result.expect("Fail to read record");
        if i == 1 {
            // number of sample in the vcf
            sample_count = usize::try_from(record.sample_count()).unwrap();
            break;
        }
    }
    println!("[STDOUT]: Total samples in vcf: {}",sample_count);
    // establish long standing vector of sample count length with fields for ref, alt and nmissing
    // if byvector == true {
    //     let mut allele_cnt_vector = vec![[0,0,0]; sample_count];
    // } else {
        // create a vector of sample count size 
    //let mut allele_cnt_vector = std::iter::repeat_with(|| Alleles::new()).take(sample_count).collect::<Vec<_>>();
    // }
    let mut allele_cnt_vector = Vec::with_capacity(sample_count);
    for _ in 0..sample_count {
        allele_cnt_vector.push(Alleles::new());
    }    

    // iterate through each row of the vcf body.
    for (i, record_result) in bcf.records().enumerate() {
        let record = record_result.expect("Fail to read record");
        let mut s = String::new();
         for allele in record.alleles() {
             for c in allele {
                 s.push(char::from(*c))
             }
             s.push(' ')
         }

        // Counting ref, alt and missing alleles for each sample
        let gts = record.genotypes().expect("Error reading genotypes");
        for sample_index in 0..sample_count {
            // for each sample
            //println!("{:#?}",allele_cnt_vector[sample_index]);
            for gta in gts.get(sample_index).iter() {
                // for each allele
                match gta.index() {
                    Some(0) => allele_cnt_vector[sample_index].sref += 1,  // reference allele
                    Some(_) => allele_cnt_vector[sample_index].salt += 1,  // alt allele
                    None => allele_cnt_vector[sample_index].snmiss += 1, // missing allele
                };
            }
        }
    }
    let mut cnt:i32 = 0;
    for sample_i in allele_cnt_vector {
        let refout:f64 = sample_i.sref as f64;
        let altout:f64 = sample_i.salt as f64;
        let nmiss:f64 = sample_i.snmiss as f64;
        let altfreq:f64 = (altout/(refout + altout + nmiss)) as f64;
        cnt += 1 ;
        println!("[STDOUT]: allele counts for sample: {:?}\nref:{:?}\n,alt:{:?}\n,missing:{:?}\nalt_freq:{:?}\n",cnt,refout,altout,nmiss,altfreq);
    }
    println!("[STDOUT]: total count of samples output: {}",cnt);
}