/*
 * @Description:
 * @Version: 1.0
 * @Autor: z.cejay@gmail.com
 * @Date: 2022-11-02 18:38:14
 * @LastEditors: cejay
 * @LastEditTime: 2022-11-02 23:10:42
 */

use std::net::Ipv4Addr;

pub async fn get() -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.ipify.org").await?.text().await?;
    let ip = resp.parse::<Ipv4Addr>()?;
    Ok(ip)
}
