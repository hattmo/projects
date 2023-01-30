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
        password: "$1$ID86MhWM$HinsS6ZP9ooFI5WqQXkvB0"
    ssh:
        install-server: yes
        allow-pw: no
    storage:
        layout:
            name: lvm
    network:
        version: 2
        renderer: networkd
        ethernets:
            default:
                match:
                    name: e*
                dhcp4: yes
                dhcp-identifier: mac
    late-commands:
        - "echo 'TrustedUserCAKeys /etc/ssh/ca.pub' > /target/etc/ssh/sshd_config.d/base.conf"
        - "echo '${ca_cert_pub}' > /target/etc/ssh/ca.pub"
        - "echo '%sudo ALL=(ALL) NOPASSWD:ALL' > /target/etc/sudoers.d/base"
        - "curtin in-target --target=/target -- chmod 440 /etc/sudoers.d/base"