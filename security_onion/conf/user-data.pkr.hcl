#cloud-config
autoinstall:
  version: 1
  locale: en_US
  keyboard:
    layout: en
    variant: us
  identity:
    hostname: ubuntu
    username: matthew
    password: $1$ZOQEQ7bK$6VocS7h4vqsf0IF7SIE7e0
  ssh:
    install-server: yes
    allow-pw: false
  early-commands:
    - sudo systemctl stop ssh
  late-commands:
    - echo "%sudo ALL=(ALL) NOPASSWD: ALL" > /target/etc/sudoers.d/nopasswd
    - echo "TrustedUserCAKeys /etc/ssh/ca.pub" > /target/etc/ssh/sshd_config.d/trust_ca
