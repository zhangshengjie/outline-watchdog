/*
 * @Description:
 * @Version: 1.0
 * @Autor: z.cejay@gmail.com
 * @Date: 2022-11-02 13:32:29
 * @LastEditors: cejay
 * @LastEditTime: 2022-11-03 00:20:42
 */

use std::net::Ipv4Addr;
use structopt::StructOpt;

mod aws_route53;
mod current_ip;

#[derive(Debug, StructOpt)]
#[structopt(name = "outline vpn watchdog", about = "############")]
struct Opt {
    #[structopt(
        short,
        long,
        help = "hosted zone id",
    )]
    zone_id: String,

    #[structopt(
        short,
        long,
        help = "your hostname",
    )]
    hostname: String,

    #[structopt(short, long, help = "region", default_value = "us-east-1")]
    region: String,

    #[structopt(
        short,
        long,
        help = "your aws access key id",
    )]
    access_key_id: String,

    #[structopt(
        short,
        long,
        help = "your aws secret access key",
    )]
    secret_access_key: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let ip: Ipv4Addr;
    loop {
        let _ip = current_ip::get().await;
        if _ip.is_err() {
            println!("get current ip failed: {:?}", _ip);
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            continue;
        }
        ip = _ip.unwrap();
        println!("current ip: {}", ip);
        break;
    }

    // init aws route53 client
    let aws = aws_route53::AWS::new(&opt.region, &opt.access_key_id, &opt.secret_access_key).await;

    // get A record from aws route53 , retry 10 times
    let mut record: Option<Ipv4Addr> = None;
    for _ in 0..10 {
        let _record = aws.get_record(&opt.zone_id, &opt.hostname).await;
        if _record.is_err() {
            println!("get record failed: {:?}", _record);
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            continue;
        }
        record = Some(_record.unwrap());
        break;
    }
    if record.is_none() {
        println!("get record failed");
        return;
    }
    let record = record.unwrap();

    // compare ip and record
    if record.eq(&ip) {
        println!("ip and record are the same");
        return;
    }

    // update record , retry 2 times
    for _ in 0..2 {
        let _re = aws.update_record(&opt.zone_id, &opt.hostname, &ip).await;
        if _re.is_err() {
            println!("update record failed: {:?}", _re);
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            continue;
        }
        println!("update record success,from {} to {}", record, ip);
        break;
    }
}
