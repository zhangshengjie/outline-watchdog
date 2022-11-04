# outline watchdog

## install

```bash
sudo bash -c "$(wget -qO- https://raw.githubusercontent.com/zhangshengjie/outline-watchdog/main/install.sh)" install.sh --zoneid "xx" --hostname "xx" --access_key_id "xx" --secret_access_key "xx"
```

### start.command
```bash
REGION="ap-xxx-1"
INSTANCEID="i-xxx"
HOSTNAME="xx.xx"
HOSTEDZONEID="xxxx"

#################################################
sudo echo "start"

echo "Starting instance ${INSTANCEID} in region ${REGION}"
aws ec2 start-instances --region ${REGION} --instance-ids ${INSTANCEID}

echo "Waiting for instance to start"
aws ec2 wait instance-running --region ${REGION} --instance-ids ${INSTANCEID}

# get public ip
public_ip=`aws ec2 describe-instances --region ${REGION} --instance-ids ${INSTANCEID} --query 'Reservations[0].Instances[0].{"PublicIP":PublicIpAddress}' --output text`
echo ec2 public IP: $public_ip
CHANGEBATCH="{\"Changes\": [{\"Action\": \"UPSERT\",\"ResourceRecordSet\": {\"Name\": \"${HOSTNAME}\",\"Type\": \"A\",\"TTL\": 60,\"ResourceRecords\": [{\"Value\": \"${public_ip}\"}]}}]}"
#echo $CHANGEBATCH
echo "Updating DNS record for ${HOSTNAME} to ${public_ip}"
UPSETCOMMAND="aws route53 change-resource-record-sets --hosted-zone-id ${HOSTEDZONEID} --change-batch '${CHANGEBATCH}'"
echo $UPSETCOMMAND
eval $UPSETCOMMAND


while true; do
    echo "WAIT DNS UPDATE TO ${public_ip}" 
    #sudo killall -HUP mDNSResponder
    dig +short ${HOSTNAME} | grep ${public_ip} && break || echo "DNS not updated yet" && sleep 10 
done

exit 0
```

### stop.command
```bash
REGION="ap-xxxx-1"
INSTANCEID="i-xxxxx"
echo "Stopping instance $INSTANCEID in $REGION"
aws ec2 stop-instances --force --region ${REGION} --instance-ids ${INSTANCEID}
# wait for stop
echo "Waiting for instance to stop"
aws ec2 wait instance-stopped --region ${REGION} --instance-ids ${INSTANCEID}
echo "Instance stopped"

exit 0
```