## Octopus NEAR Indexer Kafka

### 0.0.1

此为本服务第一个版本

默认创建两个 kafka topic 以测试网高度 80989087 为例包含 1 个 block，4 个 chunk。



#### blockchain-near-block

```
{
  "author": "node1",
  "chunks": [
    {
      "balance_burnt": "22318256250000000000",
      "chunk_hash": "7bViTmrhRfqS9gLM4rjvK8kG2m7eKTMrSkF4igpKXA6h",
      "encoded_length": 171,
      "encoded_merkle_root": "D1SqTWKAWkFKGznEAhKEmh1txMuEdmvKwpoAHBHxLwLE",
      "gas_limit": 1000000000000000,
      "gas_used": 446365125000,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "5ev3u5LdwCp57zBuTpGASbAzcWoA8ZfvSBQUiQWJj7ET",
      "outgoing_receipts_root": "sYHH3rsTfhjnF8RRV5sqxTZttwWFtaZtdYqjYaWbcHe",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "4VZUW6BG28jR6EGfuvX8RLu6SJSjveVRpn5Z48devjhY",
      "rent_paid": "0",
      "shard_id": 0,
      "signature": "ed25519:5zWoL71MFDvqfLvETtjJQANFgk8i51SmfBUgCMdpk2Y78EcGQtwxDEDb54LCnDtPXeq4f2QpFtGP8Ld7gGfQ5fRp",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    {
      "balance_burnt": "0",
      "chunk_hash": "EdHyHVMYsa9tuk3fQT5dZzU4xrh2m9586p4kkT3VwfNL",
      "encoded_length": 8,
      "encoded_merkle_root": "5TxYudsfZd2FZoMyJEZAP19ASov2ZD43N8ZWv8mKzWgx",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "FguxodrwWngChBbq7kARUpd28v4yntGqTVU1CNYNseco",
      "rent_paid": "0",
      "shard_id": 1,
      "signature": "ed25519:4vW8Fp9a5DzCMk8AZhVAENN8iaytwoQeC5gQhP98zVUhozVJRfCz5Ek4oAyZ3W3cnsisNrfqQkZt4obWfenJx15J",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    {
      "balance_burnt": "0",
      "chunk_hash": "8wE3jXzLB8YfJ2MZHeKTqV7tV1a3dZPDgiRkiGvptwNh",
      "encoded_length": 8,
      "encoded_merkle_root": "5TxYudsfZd2FZoMyJEZAP19ASov2ZD43N8ZWv8mKzWgx",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "BdeWCWuSaAZjLi1DjP8GZfhxdgA1Zt5dHcKPrErh87Pq",
      "rent_paid": "0",
      "shard_id": 2,
      "signature": "ed25519:5qZBH44LHxvGLCrbfEaPrTNGsse2oe8isHZjPRXrD5VK7xhZb2ymSNzUGvtnnjCgdREKvEnDNaSL3z7xLPinrh2U",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    {
      "balance_burnt": "0",
      "chunk_hash": "4KVqgeMLmVqBMU7FAobXpUQA4Urx1A2TgH8Nk8VCiTXc",
      "encoded_length": 105108,
      "encoded_merkle_root": "BD9n6YuB7cUBhD8augybWaWDAG9zW3h1FJeUkdKCNDzi",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "6ucAhmaYtvta8oiKBYRe2qFkLftKmqMdZmWHinri4nVc",
      "rent_paid": "0",
      "shard_id": 3,
      "signature": "ed25519:Xr1UAw3uHFT1RhvZHaYBQ3Qrd7ri6GnKAdkRefnUAJxTfdaPkqsqYMC7f8DHwjrfAM9wg97WmvV8Wy9ajAWWXbk",
      "tx_root": "BU1bsReKxa6yDHe96jD5Nrstk3XXMLLvEEcE8mmsiAnV",
      "validator_proposals": [],
      "validator_reward": "0"
    }
  ],
  "header": {
    "approvals": [
      "ed25519:59dib5ESA6uMZx1eGkwNEDcZc2XdUviiir1ghyuUnrkkKg6H2ytfFEVGAEQFj4EHB2Cssqy5Yowczdys9zmX7ZE5",
      "ed25519:59dib5ESA6uMZx1eGkwNEDcZc2XdUviiir1ghyuUnrkkKg6H2ytfFEVGAEQFj4EHB2Cssqy5Yowczdys9zmX7ZE5",
      "ed25519:59dib5ESA6uMZx1eGkwNEDcZc2XdUviiir1ghyuUnrkkKg6H2ytfFEVGAEQFj4EHB2Cssqy5Yowczdys9zmX7ZE5",
      "ed25519:59dib5ESA6uMZx1eGkwNEDcZc2XdUviiir1ghyuUnrkkKg6H2ytfFEVGAEQFj4EHB2Cssqy5Yowczdys9zmX7ZE5",
      null,
      null,
      "ed25519:5haK7KqvY7n8qfqyiS1bUEj9Lc9nneg6pkGhznCq4kxz2x61GoNSX2mHhFuWYCQJ43RFpZLiqkjXWp5tDvhDd411",
      "ed25519:56CcGzP1oad9BKvPR6CovFyCyW11sbq9MWsQE3z4Sw8SG5MW6PeTqwPiKYv7cb4CiAmCh6NtV9wS1HjQrwzNRQPY",
      null,
      "ed25519:5yp1wazqzRaDA86JeMwm9C3xuLJZbWsZLAn6bYd1dY18xniQ6aevmXhynf2JXXuqCYjhjKuYfsg5K7dqxsExAcrJ",
      null,
      null,
      "ed25519:2yENNpfJSYTJnF8RHZgdSCzsaEgfc3sqkrzKC5FeiZL5QJbCVAaoFb5YWRvr6BHTq5mxVrGUYaFoJ2bNdrxnYTcq",
      "ed25519:3yccSapc4m5AbX9Jh3He6K7poqGWvdz75URdnMSF7YyDvqeaMF9B4UjpeHqiLZ9QwZV7gRVZ7sq3Nfk774uzsVLr",
      "ed25519:33T3QGeaLdVxPnZHYqniP5mp1TnPgxaFibZhEYGmpSYqwcmN1FtTGJeiCVDLocoPqbQCV8YhwWGKNZKGawWfWB3b",
      "ed25519:5TZwTUY7fDpyQUicCs4e27eovA6M49vX5xTvkyyvg75PykZaufi1Kd6r2Y4UzscuggPuHuqZSrXiA29fE3LfA8dx",
      null,
      "ed25519:5cCwmaiSZ5nidKAL8NEskk8YDPYbJtcR14TQEHbhYfxrQdCUTxHJH4zYFe9EuaDwgjXcDv4s4YSAiqsdQxEYSKZC",
      "ed25519:43gs7H1XH1HjUArRFuZ15trC6V8rgsBBoSgMirgnUkn8bUWdQE82iE2yUJTtdh4HckNnknEjSRjXV5Hw1qYooiHn",
      null,
      "ed25519:GXQnRP3PfLRWnuXuNNXZeMFRU4yoVXSri9zRBgFKYtckpdPe83AkuCfhQnoRigwiFtN3Aci6kUGEE4VHXQq3cDp",
      "ed25519:4QZSXgAj9u7XC2HrRddLYAawSL9cxTjbY41KNU4cpogfdwUX8pkdzfcXXjkuwm1HtT42U2WA8dfz4nqS499b3Dnx",
      null,
      "ed25519:dspXtmJFVzKxtwmLC1ETWAX5juTd1eqWT1amTxY8yMFpYaWXgEXmnvtmYYXXbuwzteoBNNExrGgwUnawRHrKRbH",
      "ed25519:gENCffkjFXiLuoddBcpFLzEwDwxHpVfwb1Uon8ppGVFZ8n1q8VZRajoBhibUf6ogr7gYV7twUkb5h3fnTQ45S6r",
      "ed25519:34xA7yahRT71y26SCG5vyZwtjSEh99g96Y3Z8Nd7NrishMLgJTA9xvCcG4myLvxmqTCAMej3wr8GKMaTw2nuCxur",
      "ed25519:38oFQMSiNYdVipmPw62vk1W9p1TprSM7feXtQJzdmYCCwQUkdw6dWgaiYsoFatRXYLa3wsEoPfurcwxTFXp6AkQo",
      "ed25519:5PSR2CbGAPow4Myjjpg2jpiEsuGg6LD3DbChZV6w6JMYrLvVGrjCrYYi3a9NyJcrKFACXDeLobU3mtBA4E6hUV3B",
      "ed25519:5g35CYVZpm4hAC99Cc7fkp9ohBoN2WpEoyUzWQuRRkA3PsNLgdXD3PND4KpE3hwNpyREYCB5z2W3w2sqzyXzvaB7",
      "ed25519:DuMDtwHeXmEach76YqrxT8CKuSeNN6a4NurANfkfgirHoVspSGbK2i76ZqeyHapjxc2aqRfL8DVx9X8RDwp5V4N",
      "ed25519:Zjnw7a6UjGjudQkESmkRUFm8d4hUyHn7F1GzNp8h13gzA1FCXnNWvnnBeFhU2NpsSTh75CigXrt4f72pDhvZ4bS",
      "ed25519:59ExBWqonYi3LyFCfeC54Z81YiNb7diocob5EvJR61rhJWhaUwdXNN1DDPBxDFUr8VZTCP6SuW5Yab9Wegp61h7b",
      null,
      "ed25519:3meANNMLFnAzF5um4YCt9R4qTW2BSrbSMmW3GbkESo32WhSFg5WWjCN1i8LC8nPXj9Z9ExJLew6ytKCWLCYnrfWB",
      null,
      "ed25519:5sWVsh9aFTLrMza89oRWcvGnBve3r9zysPWbmT5X127mzfophU92d8KUvqSmpMYoJFysQtdHuUyCjXVkAGRssJeK",
      "ed25519:4ogdvh49pCiMQtkC45zdvcdSJEUuB9t3ECUB4tqjiWc8or5wWH2A2W6GGKAEeN2aHRYzuSCcJ2tT2vm39Y49WAPc",
      "ed25519:2SgK3H5NqbBQsZkn5FuShqrrLFSxQ5t3pqTar9aZJRdQDLPAUr5QSN4TsCcXsjzHqKScBa5taF2KFt6m8nUG63sN",
      "ed25519:LNeJUC4rU3psNcZ1tPBy8QFbRDM3UzKfnrBBCVFtJJJPDy71UdNubHhRn1jddC1bFr2WcKLN2z5vCMaWMT5ktvZ",
      "ed25519:3chDMrMQLfKJunkjF8GbbWMquJjykC7GRsxDExQuWQBRstLw75mWSeLpJqQ4MqTL9KbxL9rem6m8DnUir189Kzft",
      "ed25519:Rgy9zNQNMJ4pt7UnfT6fiRgt9QkL67JuRbYDEMo4CXGXPqdaRFmi52m1bZy5uf5H5Rm1F833Zkcon2grYyNL7ja",
      null,
      "ed25519:kRNLvkggv5dqFL8vjLKNTRaYsauBRqaE4gMftGCj5RkbMuS5BhAAxTMV7PtY3SPBD5JXp3eCSR7xnRrpxKZywmw",
      "ed25519:dXGHUbSTXUu6SdZYShEcyr1uoHzpY5VUHMnd2QHnXgWNWAAVzk91FxPTt7PrEJ4EPhuwE7d6mDGqKvwDbvDMKP7",
      "ed25519:2ddcibAYNSmT12SWK7HzSV9M5fc6U7zkfwP9qsPYeZMAh3VeL1TpXdhJEvEodS2BiBAedMAm66q2TDcbuif14fhm",
      "ed25519:2Kk5oqBFKRqLJ3wALxaA4spWp8NqsWNthwHjaSApKeH7HCQ7WpfoZVspBTR2dD53Q37cvghmzPLGsGg1Q94MTsdd",
      "ed25519:5qANhrukSotSaHAMdvP5BQMpF7J5FcQbNvaH5XTt3ZsujGMe9kbwH5fK1E7sQe2UrYqabxb1A6T8kpBamTnqJTAK",
      "ed25519:QMb4YGd5JJwipD6mna5C2BkPvQbHjjyrfXMUZNw9ijRKtVAy5eGgrVKVGmAX7nCQXfw2EGG1bN88mvrDCdeqApL",
      "ed25519:3bnimTL1ya6ALMa1SxyqFX13RBUamTcxXn4tMtEYAMnrY2cjPchxZsZbbcuTeKN8tpo2fYFL9jyAnTF9VYkD9nEb",
      "ed25519:26pdLJ6ZEo5TjFYGHhAmvokAzca77TPnLq8VQwLHXGRAwPNm6TzywkgHZ22NCHPPS1Qz6LsTMkr9MyrvQBrwsCWU",
      "ed25519:2qmXdFJWUuncG512PwwL6YU6N8TwxB2eufh6cJ9jqEAeBwSB4ZGqccjM3qLauCtdvMMMzmmT4RCqbZwJNAD1cZSY",
      "ed25519:2D74o2QvCn39bqraHsQySjTwXv6APVf7jYegrY4X2fZDKmaZ9yEhFCYmStk9kz3WtkA59bcm69nA9hZ3WR6m6Yj2",
      "ed25519:3dXgDqKBw4xeWAqKx6dnbHSXjm3soro8EUksGh6fFHbSGJCZPtLFPjkpVQGEtWgiFn2NMiEUtycaUd9W2s2U2dVn",
      null,
      "ed25519:2k2pdYQqcnaQtbVuX19LDCFA2RzwTHfeUQr7KZyD65PmEvzXwcatUrFrCxQn9qc46xAwALf7Mrhqf4tmfFJ487N4",
      null,
      null,
      null,
      "ed25519:21AWG4G9nSNj731Xsi6NpKZd29nC3vhkT6eWuEwif8THPp6oW5MHWWj4sCXJqLk8YGac16ic77uC8y9k6xEMS9V2",
      "ed25519:5cAXFAenKQ52aAMhm4Hw5Uj9VfQTSpr2UgmGPtCbz3GhN13z7UazKh8AGkvfP3YgzTEBBeYhzhsP4481AW2L8xap",
      "ed25519:4vk49LuBqgEkXGoirN2dxKJVMj2GNsXVFXGFcXEZv9F7z3k24qdX9Ckd77xUAEodZ5MsnLRAaLDudo72QjrJPEqt",
      "ed25519:2AV8D8woyfFAvFwdaycYaSN4EUkh8NKc7PXVkodHqGfxaM9kfELKrbazuugJRq7Adchdei52nR29Y6oZTX7fPsNv",
      null,
      null,
      "ed25519:5cjjfGkdm5vt9Y81fMeHqPm8hKKHyF6SUiA9zxoNxQrr24d6xYbv1s9caV2kJn7XWYDx2hbsAPzR9zRb4fAYFmPb",
      "ed25519:3VXoA4Xxbu1PiyeYwp1MsERe4LYXKeVFbFM1DrgL7tm7pXgihAUK6ZGUSZadbYSrSpheucoRfj98Yn7bXdD9EZZz",
      "ed25519:3G33HjhjfNhvVNL1K3EzdVpcazYP7yXEt47fecZuUnqkT7zR7PkUosSLDJJ7267QgChCRDjAv3U3iwNwkoM3ozF9",
      null,
      null,
      "ed25519:4eLj8cFsbASbVEjWJKaQYaSM71GKM4oBcKePh51uMqDwJ1H5ZRY1soKSiLfFk3yAAbVPHnvnteAb7D3EAAuHesgY",
      "ed25519:4F7XfNHM4ZRg2HRDid76pGvV3vkprXjWyrgH6jgqoMpZqa4oqr1MbHeEwqmhhzqryG2Q4Mm4Yv929VtSoMML9Yfe",
      "ed25519:2KSaAcT7Bhfhy9GiEQoUqsxdpJPnHvWPW815fSDnmZzNpuMKKyQqKecCnEL6pPqmctZRwZt7nLy2erXmXduF9AKB",
      "ed25519:mRM4wcY3hychrUxiySLr2RoZkWC1N4TC1KmsTgsaMB2U2CEHz6gLwzuk1dKVBnvBxnZjTxPwcEs2dzu1fDMa8xf",
      "ed25519:21TiDnDw42Uk3kPG9i73k97T4Rk9ZVU9Cp54Ci9CHVFGhqsCuEAWgcxbvGPWWusWxnhQyiNZrN7DCeXmwwLcdNgG",
      "ed25519:4b1VPSkdA4DsFv1cFdJbWQCmYomdjz4fHXfu1tw5isugX8Lw6yFJcHnqMuhCCdyiP5E4U5KqRATf1gbDoikVLr2D",
      "ed25519:4TNBYFhki1M4myJzR3m1tniUoTDnVi2hVBxcYv5K4doCm44Zhy4MkCHhwGiAuxZfwqoRZcTiqwCxqCrL4qYq3n3F",
      null,
      "ed25519:2BmXcjqQRf5gyXbrv2GE1AVvwUzEckE2G15BdGn64UKhGQtsxqsURmezSmcJmGksxNBF9c3me6mNyWw14xudRfJQ",
      "ed25519:2wfjFZmeDS7eF9w4yEBwDUPC3aUKbKUnCXQM81nArCpKwYEENqDfnrBr549Znu2bdaRW32h5z7uknGHBAohcrLsr",
      "ed25519:31mJV1h3XaVESxhnPTvG8e8MEjVhxXNQxFGobVg4gvXdW9Noi2TsoejjioPchQESK4VdCUXVNWhXcsaS9AEVAR5r",
      "ed25519:23poqfFKGmvDawP7tMgzFLw4i2qRMw59nP8XWyVxz5Fx1DMaJcpNC79xeD7nqFpGfoh26z8r2vtn6cMn1ghUExuF",
      null,
      "ed25519:3ZtFCdgGMGXWZaRmbSXKVJ5gxPkTBFWNBZhrGjC3scjyMUXN5cV9Uoycs22GaPchZSuNRYdHT4yp94mKGBLAnX2X",
      null,
      "ed25519:4WeyDzL7aLMtuNSmyvnSoi9QhxSCBdAqVu3yLjsFpmVKKnzMHvQLTEGj68MREuAyuECKzhGkG2LTLvG13wJv2Rqt",
      null,
      "ed25519:5Tj96jtxDwjzQcP7hppHQjQg1kUmmMe8x7nSWgdNDfhHDtRDwiF5UCMj6ryChKWha85umpRk55oPc3krjhFJrTyy",
      "ed25519:5MkA3fb9RzMmF1W8EptRkeQCLBG9izumW63CGGHMJVfWk6aerKvyfxPXc4AaEZpSNncqr8cRpPQ7TJWENLMhkAbY",
      "ed25519:4TGk9ycHAwYa9rq4GPX535cD4v66U4K8RqkWR4ViQFxKUpPyWkLjEujjY7NM2G1c3c6vBLM4ZwmW8pVtEqQyKoFr",
      "ed25519:kZeWqowdecnU7DGXpztgMGhx162u96GZB8SNs5fb5PYGM726bX5xhWYEGUEJcH864ToWMUvfWcZyqq8RBxhUV38",
      "ed25519:3CLmwuUALy2EkbrKbLKcWXTrdqnWhyWEB7BuLhfJWx8eaAE7auXdtqiD1hQw9UYo8CQ1dcGSx4ba9uysj5sGUZmf",
      "ed25519:5bh3ohqbjaYLgwHE8fvrStAfs8fxuzGFztzXgybmDCBp96A5HpH1NLAYzAjiibrLE1DFq6Sg2RRiBM63VuG4i2kp",
      null,
      "ed25519:3B5U7RYp91patNnK7indosdJHZW4hCNz7FfCieoMUYTL4jM6J7ocNFqcy7ujDzknRpJj73HK9AzTcU3Gei3dGeAx"
    ],
    "block_merkle_root": "HNxGWE3c5j5XvBJ5XzMPjs4z3dqKnyXcF7yGzoP18WKZ",
    "block_ordinal": 36911650,
    "challenges_result": [],
    "challenges_root": "11111111111111111111111111111111",
    "chunk_headers_root": "719aZhjKehcu2rvevyxxw12d2vUZM6YQTfwBy2guWUs1",
    "chunk_mask": [
      true,
      true,
      true,
      true
    ],
    "chunk_receipts_root": "J3JJTaGFzKLuyYoeJqU2Ko4txo2437K3jfowU89UQdok",
    "chunk_tx_root": "6oFWDA2uvaJWmLMiHk7cvX8yxRK4nvSJmCnn45sMtumC",
    "chunks_included": 4,
    "epoch_id": "74QvBnxTUts7PXdhqXYjnk9cofgeVy4rFYRkRGbpf4X9",
    "epoch_sync_data_hash": null,
    "gas_price": "100000000",
    "hash": "AmxdtieJo1WPrX14VRpQJR3QDAapWgVo4PJweA1uQ24d",
    "height": 80989087,
    "last_ds_final_block": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
    "last_final_block": "8jDZoCb8UWuPzUbhqbMtdUa1xNw8T5rsL6brjnxAYn8s",
    "latest_protocol_version": 51,
    "next_bp_hash": "4VT4cEHZRukrxmsqVxXi5GyaiJyzGrfAvkCBPjwXgDSe",
    "next_epoch_id": "6aUWeNxAyH7VX4HCLFcyh8FhG5T79Hufhe1aa4Ubei5J",
    "outcome_root": "AvxS6XH82ULPCbTZ9NcUzuQHGSgdjZpEXDhPFeYae6Ts",
    "prev_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
    "prev_height": 80989086,
    "prev_state_root": "29VfCbTujR9ZoqG9rTU6NKyMhWUZtHd6XfTQsdtCY8zw",
    "random_value": "e6KrYmKrTi44j1NiWCkdxrSZ4a1KaLvYQWs9msquXWY",
    "rent_paid": "0",
    "signature": "ed25519:2YtyyykesvNWWUbjXLHqCvFWtyNVeMx1jQt8xWvkkXhjAe4Ey6CENJi1Nttj5eeRWos1mdSzheCEa1JKWb7zaaJB",
    "timestamp": "1643758250030548411",
    "timestamp_nanosec": "1643758250030548411",
    "total_supply": "2177640053380057023467327633410584",
    "validator_proposals": [],
    "validator_reward": "0"
  }
}
```



#### blockchain-near-chunks

```
{
  "chunk": {
    "author": "node1",
    "header": {
      "balance_burnt": "22318256250000000000",
      "chunk_hash": "7bViTmrhRfqS9gLM4rjvK8kG2m7eKTMrSkF4igpKXA6h",
      "encoded_length": 171,
      "encoded_merkle_root": "D1SqTWKAWkFKGznEAhKEmh1txMuEdmvKwpoAHBHxLwLE",
      "gas_limit": 1000000000000000,
      "gas_used": 446365125000,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "5ev3u5LdwCp57zBuTpGASbAzcWoA8ZfvSBQUiQWJj7ET",
      "outgoing_receipts_root": "sYHH3rsTfhjnF8RRV5sqxTZttwWFtaZtdYqjYaWbcHe",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "4VZUW6BG28jR6EGfuvX8RLu6SJSjveVRpn5Z48devjhY",
      "rent_paid": "0",
      "shard_id": 0,
      "signature": "ed25519:5zWoL71MFDvqfLvETtjJQANFgk8i51SmfBUgCMdpk2Y78EcGQtwxDEDb54LCnDtPXeq4f2QpFtGP8Ld7gGfQ5fRp",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    "receipts": [
      {
        "predecessor_id": "system",
        "receipt": {
          "Action": {
            "actions": [
              {
                "Transfer": {
                  "deposit": "117179811448765875000"
                }
              }
            ],
            "gas_price": "0",
            "input_data_ids": [],
            "output_data_receivers": [],
            "signer_id": "amberdata.testnet",
            "signer_public_key": "ed25519:HakbHwGjynqqEJ3GMCJwZJAUQenqQMDfA8b4sPQMBtnz"
          }
        },
        "receipt_id": "43AdHUEWrfEB9kBAapZFqQhJhRrsZ2wGs87FGRCyV7mT",
        "receiver_id": "amberdata.testnet"
      }
    ],
    "transactions": []
  },
  "receipt_execution_outcomes": [
    {
      "execution_outcome": {
        "block_hash": "AmxdtieJo1WPrX14VRpQJR3QDAapWgVo4PJweA1uQ24d",
        "id": "43AdHUEWrfEB9kBAapZFqQhJhRrsZ2wGs87FGRCyV7mT",
        "outcome": {
          "executor_id": "amberdata.testnet",
          "gas_burnt": 223182562500,
          "logs": [],
          "metadata": {
            "gas_profile": [],
            "version": 1
          },
          "receipt_ids": [],
          "status": {
            "SuccessValue": ""
          },
          "tokens_burnt": "0"
        },
        "proof": []
      },
      "receipt": {
        "predecessor_id": "system",
        "receipt": {
          "Action": {
            "actions": [
              {
                "Transfer": {
                  "deposit": "117179811448765875000"
                }
              }
            ],
            "gas_price": "0",
            "input_data_ids": [],
            "output_data_receivers": [],
            "signer_id": "amberdata.testnet",
            "signer_public_key": "ed25519:HakbHwGjynqqEJ3GMCJwZJAUQenqQMDfA8b4sPQMBtnz"
          }
        },
        "receipt_id": "43AdHUEWrfEB9kBAapZFqQhJhRrsZ2wGs87FGRCyV7mT",
        "receiver_id": "amberdata.testnet"
      }
    }
  ],
  "shard_id": 0,
  "state_changes": [
    {
      "cause": {
        "receipt_hash": "43AdHUEWrfEB9kBAapZFqQhJhRrsZ2wGs87FGRCyV7mT",
        "type": "receipt_processing"
      },
      "change": {
        "account_id": "amberdata.testnet",
        "amount": "131983270139259274352532090",
        "code_hash": "11111111111111111111111111111111",
        "locked": "0",
        "storage_paid_at": 0,
        "storage_usage": 346
      },
      "type": "account_update"
    }
  ]
}

{
  "chunk": {
    "author": "node1",
    "header": {
      "balance_burnt": "0",
      "chunk_hash": "EdHyHVMYsa9tuk3fQT5dZzU4xrh2m9586p4kkT3VwfNL",
      "encoded_length": 8,
      "encoded_merkle_root": "5TxYudsfZd2FZoMyJEZAP19ASov2ZD43N8ZWv8mKzWgx",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "FguxodrwWngChBbq7kARUpd28v4yntGqTVU1CNYNseco",
      "rent_paid": "0",
      "shard_id": 1,
      "signature": "ed25519:4vW8Fp9a5DzCMk8AZhVAENN8iaytwoQeC5gQhP98zVUhozVJRfCz5Ek4oAyZ3W3cnsisNrfqQkZt4obWfenJx15J",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    "receipts": [],
    "transactions": []
  },
  "receipt_execution_outcomes": [],
  "shard_id": 1,
  "state_changes": []
}

{
  "chunk": {
    "author": "node1",
    "header": {
      "balance_burnt": "0",
      "chunk_hash": "8wE3jXzLB8YfJ2MZHeKTqV7tV1a3dZPDgiRkiGvptwNh",
      "encoded_length": 8,
      "encoded_merkle_root": "5TxYudsfZd2FZoMyJEZAP19ASov2ZD43N8ZWv8mKzWgx",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "BdeWCWuSaAZjLi1DjP8GZfhxdgA1Zt5dHcKPrErh87Pq",
      "rent_paid": "0",
      "shard_id": 2,
      "signature": "ed25519:5qZBH44LHxvGLCrbfEaPrTNGsse2oe8isHZjPRXrD5VK7xhZb2ymSNzUGvtnnjCgdREKvEnDNaSL3z7xLPinrh2U",
      "tx_root": "11111111111111111111111111111111",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    "receipts": [],
    "transactions": []
  },
  "receipt_execution_outcomes": [],
  "shard_id": 2,
  "state_changes": []
}

{
  "chunk": {
    "author": "node1",
    "header": {
      "balance_burnt": "0",
      "chunk_hash": "4KVqgeMLmVqBMU7FAobXpUQA4Urx1A2TgH8Nk8VCiTXc",
      "encoded_length": 105108,
      "encoded_merkle_root": "BD9n6YuB7cUBhD8augybWaWDAG9zW3h1FJeUkdKCNDzi",
      "gas_limit": 1000000000000000,
      "gas_used": 0,
      "height_created": 80989087,
      "height_included": 80989087,
      "outcome_root": "11111111111111111111111111111111",
      "outgoing_receipts_root": "8s41rye686T2ronWmFE38ji19vgeb6uPxjYMPt8y8pSV",
      "prev_block_hash": "FKWGGKhqAqVcWemxtddN4tbJHiLTukVChf3hXbWmfdX7",
      "prev_state_root": "6ucAhmaYtvta8oiKBYRe2qFkLftKmqMdZmWHinri4nVc",
      "rent_paid": "0",
      "shard_id": 3,
      "signature": "ed25519:Xr1UAw3uHFT1RhvZHaYBQ3Qrd7ri6GnKAdkRefnUAJxTfdaPkqsqYMC7f8DHwjrfAM9wg97WmvV8Wy9ajAWWXbk",
      "tx_root": "BU1bsReKxa6yDHe96jD5Nrstk3XXMLLvEEcE8mmsiAnV",
      "validator_proposals": [],
      "validator_reward": "0"
    },
    "receipts": [],
    "transactions": [
      {
        "outcome": {
          "execution_outcome": {
            "block_hash": "AmxdtieJo1WPrX14VRpQJR3QDAapWgVo4PJweA1uQ24d",
            "id": "qHPQwBsTrgqESgcEnocyuDe4nfMCcpD3sjXReVb2ka6",
            "outcome": {
              "executor_id": "signer.goerli.testnet",
              "gas_burnt": 7301987770672,
              "logs": [],
              "metadata": {
                "gas_profile": null,
                "version": 1
              },
              "receipt_ids": [
                "A6kLoKURmcMWEz4MEh9qr1w9jkZMnKvyrP3XsxhtMxsy"
              ],
              "status": {
                "SuccessReceiptId": "A6kLoKURmcMWEz4MEh9qr1w9jkZMnKvyrP3XsxhtMxsy"
              },
              "tokens_burnt": "730198777067200000000"
            },
            "proof": []
          },
          "receipt": null
        },
        "transaction": {
          "actions": [
            {
              "FunctionCall": {
                "args": "",
                "deposit": "0",
                "gas": 75000000000000,
                "method_name": "add_block_header"
              }
            },
            {
              "FunctionCall": {
                "args": "",
                "deposit": "0",
                "gas": 75000000000000,
                "method_name": "add_block_header"
              }
            },
            {
              "FunctionCall": {
                "args": "",
                "deposit": "0",
                "gas": 75000000000000,
                "method_name": "add_block_header"
              }
            }
          ],
          "hash": "qHPQwBsTrgqESgcEnocyuDe4nfMCcpD3sjXReVb2ka6",
          "nonce": 56959092698401,
          "public_key": "ed25519:Bw8EpD3JUjsnGiNiJKd5XPC4DyBy2HmUXN5DeYodENLj",
          "receiver_id": "client6.goerli.testnet",
          "signature": "ed25519:qzhqNjHvJBMEqd6t6XxqwWVNKFhqVBzADbKTHS6mryYCWKc3Q2mbGsAZ11jmpQmfacnSM7fi1o75XEz8zXbVV5s",
          "signer_id": "signer.goerli.testnet"
        }
      }
    ]
  },
  "receipt_execution_outcomes": [],
  "shard_id": 3,
  "state_changes": [
    {
      "cause": {
        "tx_hash": "qHPQwBsTrgqESgcEnocyuDe4nfMCcpD3sjXReVb2ka6",
        "type": "transaction_processing"
      },
      "change": {
        "account_id": "signer.goerli.testnet",
        "amount": "97816562010758390518172667",
        "code_hash": "11111111111111111111111111111111",
        "locked": "0",
        "storage_paid_at": 0,
        "storage_usage": 182
      },
      "type": "account_update"
    },
    {
      "cause": {
        "tx_hash": "qHPQwBsTrgqESgcEnocyuDe4nfMCcpD3sjXReVb2ka6",
        "type": "transaction_processing"
      },
      "change": {
        "access_key": {
          "nonce": 56959092698401,
          "permission": "FullAccess"
        },
        "account_id": "signer.goerli.testnet",
        "public_key": "ed25519:Bw8EpD3JUjsnGiNiJKd5XPC4DyBy2HmUXN5DeYodENLj"
      },
      "type": "access_key_update"
    }
  ]
}


```




