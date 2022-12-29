#!/usr/bin/env python

from typing import Tuple
import configargparse
from python_terraform import Terraform, IsNotFlagged
import os
import hashlib

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


def hash_folders(*folders:str):
    files = []
    for folder in folders:
        for root,_,f_names in os.walk(folder):
            for f in f_names:
                files.append(os.path.join(root, f))
    sha = hashlib.sha1()
    for file in files:
        with open(file,"rb") as file_content:
            sha.update(file_content.read())
    return sha.hexdigest()

    
        

def main():
    p = configargparse.ArgParser(default_config_files=['./settings.conf'])
    p.add("--vsphere_server",required=True,env_var="VSPHERE_SERVER")
    p.add("--vsphere_username",required=True,env_var="VSPHERE_USER")
    p.add("--vsphere_password",required=True,env_var="VSPHERE_PASSWORD")
    p.add("--vsphere_datacenter",required=True,env_var="VSPHERE_DATACENTER")
    p.add("--vsphere_datastore",required=True,env_var="VSPHERE_DATASTORE")
    # p.add("--vsphere_cluster",required=True,env_var="VSPHERE_CLUSTER")
    p.add("--vsphere_host",required=True,env_var="VSPHERE_HOST")
    p.add("--vsphere_folder",required=True,env_var="VSPHERE_FOLDER")
    p.add("--wan_network",required=True,env_var="VSPHERE_NETWORK")
    p.add("--gateway",required=True,env_var="GATEWAY")
    p.add("--domain",required=True,env_var="DOMAIN")
    p.add("--gitlab_ip",required=True,env_var="GITLAB_IP")
    p.add("--destroy",action="store_true")
    settings = p.parse_args()
    settings = vars(settings)
    settings["gitlab_files_hash"] = hash_folders("./ansible/gitlab")
    # certs = build_pki()

    # if len(certs) < 3:
    #     print("Not all PKI was built; dying now..")
    #     exit()
    tf = Terraform(working_dir="infrastructure", variables=settings)
    tf.init()
    if settings["destroy"]:
        del settings["destroy"]
        tf.destroy(capture_output=False, no_color=None, force=IsNotFlagged, auto_approve=True)
    else:
        del settings["destroy"]
        tf.apply(no_color=None, skip_plan=True,capture_output=False)


if __name__ == '__main__':
    main()
