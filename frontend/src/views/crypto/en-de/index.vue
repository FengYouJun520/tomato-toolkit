<template>
  <to-page title="加解密工具">
    <div class="flex flex-col">
      <t-form label-align="top" colon :data="cryptoModel">
        <t-row :gutter="[24, 24]">
          <t-col :xs="12" :md="6">
            <t-form-item label="类型">
              <t-select :options="cryptoOptions" v-model="cryptoModel.typ" />
            </t-form-item>
          </t-col>
          <t-col :xs="12" :md="6">
            <t-form-item label="盐(salt)">
              <t-input v-model="cryptoModel.cost" clearable />
            </t-form-item>
          </t-col>

          <t-col :xs="12" :md="6" v-if="cryptoModel.typ === 'rsa'">
            <t-form-item label="公钥">
              <t-textarea
                v-model="cryptoModel.publicKey"
                clearable
                :autosize="{ minRows: 5, maxRows: 10 }"
              />
            </t-form-item>
          </t-col>
          <t-col :xs="12" :md="6" v-if="cryptoModel.typ === 'rsa'">
            <t-form-item label="私钥">
              <t-textarea
                v-model="cryptoModel.privateKey"
                clearable
                :autosize="{ minRows: 5, maxRows: 10 }"
              />
            </t-form-item>
          </t-col>
        </t-row>

        <t-row :gutter="[24, 24]">
          <t-col>
            <t-form-item label="源数据">
              <t-textarea v-model="cryptoModel.source" :autosize="{ minRows: 5, maxRows: 10 }" />
            </t-form-item>
          </t-col>
          <t-col>
            <t-form-item label="加密后数据">
              <t-textarea v-model="cryptoModel.dest" :autosize="{ minRows: 5, maxRows: 10 }" />
            </t-form-item>
          </t-col>
        </t-row>
      </t-form>
    </div>
  </to-page>
</template>
<script setup lang="ts">
import ToPage from '@/components/ToPage/index.vue'
import { Encode } from '@/wailsjs/go/crypt/Crypt'
import { crypt } from '@/wailsjs/go/models'
import { MessagePlugin, SelectOption, SelectOptionGroup } from 'tdesign-vue-next'

interface CryptoProps {
  typ: string
  cost?: string
  source: string
  dest: string
  privateKey?: string
  publicKey?: string
}

const cryptoOptions: SelectOption[] | SelectOptionGroup[] = [
  {
    group: '数字签名',
    children: [
      {
        label: 'md5',
        value: 'md5',
      },
      {
        label: 'sha1',
        value: 'sha1',
      },
      {
        label: 'sha256',
        value: 'sha256',
      },
      {
        label: 'sha512',
        value: 'sha512',
      },
    ],
  },
  {
    group: '对称式',
    children: [
      {
        label: 'bcrypt',
        value: 'bcrypt',
      },
    ],
  },
  {
    group: '非对称式',
    children: [
      {
        label: 'rsa',
        value: 'rsa',
      },
    ],
  },
  {
    group: '其他编码',
    children: [
      {
        label: 'base64',
        value: 'base64',
      },
    ],
  },
]

const cryptoModel: CryptoProps = reactive({
  typ: '',
  cost: '',
  source: '',
  dest: '',
  privateKey: '',
  publicKey: '',
})

watch([() => cryptoModel.source, () => cryptoModel.typ], () => {
  if (!cryptoModel.typ || !cryptoModel.source) {
    return
  }

  Encode(crypt.CryptConfig.createFrom(toRaw(cryptoModel)))
    .then((dest: any) => {
      cryptoModel.dest = dest as string
    })
    .catch((err: any) => {
      MessagePlugin.error(err)
    })
})
</script>

<style lang="scss" scoped></style>
