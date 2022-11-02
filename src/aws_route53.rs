/*
 * @Description:
 * @Version: 1.0
 * @Autor: z.cejay@gmail.com
 * @Date: 2022-11-02 18:39:59
 * @LastEditors: cejay
 * @LastEditTime: 2022-11-02 23:07:44
 */

use std::net::Ipv4Addr;

use aws_sdk_route53 as route53;
use route53::{
    model::{Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType},
    Client, output::ChangeResourceRecordSetsOutput,
};

pub struct AWS {
    client: Client,
}

impl AWS {
    pub async fn new(region: &str, access_key_id: &str, secret_access_key: &str) -> AWS {
        {
            // write to ~/.aws/credentials
            let credentials: String = format!(
                "[default]\nregion={}\naws_access_key_id={}\naws_secret_access_key={}\n",
                region, access_key_id, secret_access_key
            );
            let fullpath = dirs::home_dir().unwrap().join(".aws");
            let fullpath = fullpath.as_path();
            if !fullpath.exists() {
                std::fs::create_dir_all(fullpath).unwrap();
            }
            std::fs::write(fullpath.join("credentials"), credentials).unwrap();
        }

        let config = aws_config::load_from_env().await;
        let client = route53::Client::new(&config);
        AWS { client }
    }

    pub async fn list_resource_record_sets(
        &self,
        hosted_zone_id: &str,
    ) -> Result<Vec<ResourceRecordSet>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .list_resource_record_sets()
            .hosted_zone_id(hosted_zone_id)
            .send()
            .await?;
        let r = resp.resource_record_sets().unwrap_or_default();
        let mut vec = Vec::with_capacity(r.len());
        for i in r {
            vec.push(i.clone());
        }
        Ok(vec)
    }

    pub async fn get_record(
        &self,
        hosted_zone_id: &str,
        domain_name: &str,
    ) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
        let list = self
            .list_resource_record_sets(hosted_zone_id)
            .await
            .unwrap();
        let _domain_name = format!("{}.", domain_name);
        for i in list {
            if i.name().unwrap_or_default() == _domain_name {
                let ip = i.resource_records().unwrap_or_default()[0]
                    .value()
                    .unwrap_or_default();
                return Ok(ip.parse().unwrap());
            }
        }
        Err("not found".into())
    }

    pub async fn update_record(
        &self,
        hosted_zone_id: &str,
        domain_name: &str,
        ip: &Ipv4Addr,
    ) -> Result<ChangeResourceRecordSetsOutput, Box<dyn std::error::Error>> {
        let resource_record: ResourceRecord =
            ResourceRecord::builder().value(ip.to_string()).build();
        let create_record_set_req: ResourceRecordSet = ResourceRecordSet::builder()
            .name(format!("{}.", domain_name))
            .resource_records(resource_record)
            .ttl(300)
            .r#type(RrType::A)
            .build();
        let change: Change = Change::builder()
            .action(ChangeAction::Upsert)
            .resource_record_set(create_record_set_req)
            .build();
        let change_batch: ChangeBatch = ChangeBatch::builder().changes(change).build();
        let resp = self
            .client
            .change_resource_record_sets()
            .hosted_zone_id(hosted_zone_id)
            .change_batch(change_batch)
            .send()
            .await?;

        Ok(resp)
    }
}
