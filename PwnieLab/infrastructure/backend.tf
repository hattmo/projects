# terraform {
#   backend "http" {
#     address        = "https://gitlab.mayhem.dev/api/v4/projects/29/terraform/state/teststate"
#     lock_address   = "https://gitlab.mayhem.dev/api/v4/projects/29/terraform/state/teststate/lock"
#     unlock_address = "https://gitlab.mayhem.dev/api/v4/projects/29/terraform/state/teststate/lock"
#     lock_method    = "POST"
#     unlock_method  = "DELETE"
#     retry_wait_min = 5
#   }
# }
