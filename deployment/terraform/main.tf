# deployment/terraform/main.tf
terraform {
  required_version = ">= 1.5.0"
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = ">= 3.90.0"
    }
  }
}

provider "azurerm" {
  features {}
}

variable "location" {
  type    = string
  default = "northeurope" # Regions with dedicated AMD SEV-SNP confidential hardware availability
}

resource "azurerm_resource_group" "rg" {
  name     = "rg-provenanceshield-v3-prod"
  location = var.location
  tags = {
    "PROVENANCESHIELD_ENVIRONMENT_MODE" = "Non-Military-Defensive-Sandbox"
    "ComplianceFramework"               = "ECHR-Civil-Protection-Utility"
  }
}

resource "azurerm_virtual_network" "vnet" {
  name                = "vnet-provenanceshield-prod"
  address_space       = ["10.0.0.0/16"]
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
}

resource "azurerm_subnet" "subnet" {
  name                 = "internal"
  resource_group_name  = azurerm_resource_group.rg.name
  virtual_network_name = azurerm_virtual_network.vnet.name
  address_prefixes     = ["10.0.2.0/24"]
}

resource "azurerm_network_interface" "nic" {
  name                = "nic-provenanceshield-prod"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name

  ip_configuration {
    name                          = "internal"
    subnet_id                     = azurerm_subnet.subnet.id
    private_ip_address_allocation = "Dynamic"
  }
}

# =========================================================================
# HARDENED CONFIDENTIAL VIRTUAL MACHINE INITIALIZATION
# =========================================================================
resource "azurerm_linux_virtual_machine" "cvm" {
  name                = "cvm-ps-boundary-prod"
  resource_group_name = azurerm_resource_group.rg.name
  location            = azurerm_resource_group.rg.location
  size                = "Standard_DC2as_v5" # Dedicated AMD EPYC hardware with SEV-SNP features enabled
  admin_username      = "secure_operator"
  network_interface_ids = [
    azurerm_network_interface.nic.id,
  ]

  admin_ssh_key {
    username   = "secure_operator"
    public_key = file("~/.ssh/id_rsa.pub")
  }

  os_disk {
    caching              = "ReadWrite"
    storage_account_type = "Premium_LRS"
    
    # Enable confidential disk encryption bound strictly to the platform hardware
    security_encryption_type = "DiskWithVMGuestState"
  }

  source_image_reference {
    publisher = "Canonical"
    offer     = "0001-com-ubuntu-confidential-vm-jammy" # Hardened, TEE-driver optimized baseline
    sku       = "22_04-lts-cvm"
    version   = "latest"
  }

  # --- MANDATORY SILICON ATTESTATION PROFILE CONFIGURATION ---
  security_type = "ConfidentialVM"
  
  vtpm_enabled     = true # Virtual Trusted Platform Module for hardware key measurements
  secure_boot_enabled = true # Block un-signed kernel or modified boot loader hijacking

  tags = {
    "Environment" = "Non-Military-Defensive-Sandbox"
    "Purpose"     = "Forensic Preservation of Violation Evidence"
  }

  identity {
    type = "SystemAssigned" # Enables Secure Key Release handshake with Azure Key Vault HSMs
  }
}
