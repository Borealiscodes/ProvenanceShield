{
  "version": "1.0.0",
  "anyOf": [
    {
      "authority": "https://azure.net",
      "allOf": [
        {
          "claim": "x-ms-isolation-tee",
          "equals": "amd-sev-snp"
        },
        {
          "claim": "x-ms-sevsnp-is-debuggable",
          "equals": "false"
        },
        {
          "claim": "x-ms-sevsnp-mrenclave",
          "equals": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        },
        {
          "claim": "x-ms-provenanceshield-purpose",
          "equals": "Non-Military-Defensive-Sandbox"
        }
      ]
    },
    {
      "authority": "https://azure.net",
      "allOf": [
        {
          "claim": "x-ms-isolation-tee",
          "equals": "intel-tdx"
        },
        {
          "claim": "x-ms-tdx-is-debuggable",
          "equals": "false"
        },
        {
          "claim": "x-ms-tdx-mrownerid",
          "equals": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        },
        {
          "claim": "x-ms-provenanceshield-purpose",
          "equals": "Non-Military-Defensive-Sandbox"
        }
      ]
    }
  ]
}
