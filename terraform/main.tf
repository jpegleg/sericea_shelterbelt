resource "vultr_ssh_key" "enre" {
  name = "enre"
  ssh_key = "YOURPUBLICSSHKEY"
}

resource "vultr_instance" "dogwood_central_us" {
    hostname = "TEMPLATENAME"
    plan = "vc2-1c-1gb"
    region = "ord"
    os_id = 2467
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "shelterbelt"
}
