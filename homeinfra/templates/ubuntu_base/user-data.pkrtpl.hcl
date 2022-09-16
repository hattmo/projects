#cloud-config
autoinstall:
    version: 1
    early-commands:
        - "sudo systemctl stop ssh"
    locale: en_US
    keyboard:
        layout: en
        variant: us
    identity:
        hostname: base
        username: admin
        password: "$6$rounds=4096$ntlX/dlo6b$HXaLN4RcLIGaEDdQdR2VTYi9pslSeXWL131MqaakqE285Nv0kW9KRontQYivCbycZerUMcjVsuLl2V8bbdadI1"
    ssh:
        install-server: yes
        allow-pw: no
    storage:
        layout:
            name: lvm
    late-commands:
        - "echo 'TrustedUserCAKeys /etc/ssh/ca.pub' > /target/etc/ssh/sshd_config.d/base.conf"
        - "echo '${ca_cert_pub}' > /target/etc/ssh/ca.pub"
        - "echo '%sudo ALL=(ALL) NOPASSWD:ALL' > /target/etc/sudoers.d/base"
        - "curtin in-target --target=/target -- chmod 440 /etc/sudoers.d/base"