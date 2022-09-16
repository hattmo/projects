#!/usr/bin/env python

import configargparse
from python_terraform import Terraform

# def build_pki():
#     """Build PKI
#     SSL/TLS keypairs for the lab domain and all created service
#     subdomains

#     @returns: tuple(CA Path, Cert Path, Key Path)
#     """
#     conf = config.parse('params.cfg')

#     tf = Terraform(working_dir="infrastructure/pki/build", variables={
#         "common_name": conf['DEFAULT']['domain'],
#         "cert_domains": config.get_domains(conf)
#     })

#     tf.init()
#     apply_output = tf.apply(no_color=None, skip_plan=True)
#     print('\n'.join(apply_output[1].split('\n')[-8:]))
#     certs = (Path("infrastructure/pki/ca.crt"), 
#             Path(f"infrastructure/pki/{conf['DEFAULT']['domain']}.crt"),
#             Path(f"infrastructure/pki/{conf['DEFAULT']['domain']}.key"))
#     return tuple(_ for _ in certs if _.exists())


def main():
    p = configargparse.ArgParser(default_config_files=['./settings.conf'])
    p.add("--vsphere_server",required=True,env_var="VSPHERE_SERVER")
    p.add("--vsphere_username",required=True,env_var="VSPHERE_USER")
    p.add("--vsphere_password",required=True,env_var="VSPHERE_PASSWORD")
    p.add("--vsphere_datacenter",required=True,env_var="VSPHERE_DATACENTER")
    p.add("--vsphere_datastore",required=True,env_var="VSPHERE_DATASTORE")
    p.add("--vsphere_cluster",required=True,env_var="VSPHERE_CLUSTER")
    p.add("--vsphere_host",required=True,env_var="VSPHERE_HOST")
    p.add("--vsphere_folder",required=True,env_var="VSPHERE_FOLDER")
    p.add("--vsphere_network",required=True,env_var="VSPHERE_NETWORK")
    p.add("--gateway",required=True,env_var="GATEWAY")
    p.add("--domain",required=True,env_var="DOMAIN")
    p.add("--gitlab_ip",required=True,env_var="GITLAB_IP")

    settings = p.parse_args()
    settings = vars(settings)
    settings["gitlab_files_hash"] = "70"
    # certs = build_pki()

    # if len(certs) < 3:
    #     print("Not all PKI was built; dying now..")
    #     exit()

    tf = Terraform(working_dir="infrastructure", variables=settings)
    tf.init()
    tf.apply(no_color=None, skip_plan=True,capture_output=False)


if __name__ == '__main__':
    main()
