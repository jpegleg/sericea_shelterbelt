#!/bin/sh

dnf remove cockpit* -y

firewall-cmd --add-service=ssh --permanent
firewall-cmd --remove-service cockpit --permanent
firewall-cmd --add-port=80/tcp --permanent
firewall-cmd --add-port=443/tcp --permanent
firewall-cmd --reload

dnf update -y
dnf upgrade -y
