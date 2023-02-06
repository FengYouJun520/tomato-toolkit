<template>
  <to-page title="加解密工具">
    <div flex flex-col>
      <t-form :data="cryptoModel" colon label-align="top">
        <t-row :gutter="[24, 24]">
          <t-col :md="6" :xs="12">
            <t-form-item label="类型">
              <t-select v-model="cryptoModel.typ" :options="cryptoOptions"/>
            </t-form-item>
          </t-col>
          <t-col :md="6" :xs="12">
            <t-form-item label="盐(salt)">
              <t-input v-model="cryptoModel.cost" clearable/>
            </t-form-item>
          </t-col>

          <t-col v-if="cryptoModel.typ === 'rsa'" :md="6" :xs="12">
            <t-form-item label="公钥">
              <t-textarea
                v-model="cryptoModel.publicKey"
                :autosize="{ minRows: 5, maxRows: 10 }"
                clearable
              />
            </t-form-item>
          </t-col>
          <t-col v-if="cryptoModel.typ === 'rsa'" :md="6" :xs="12">
            <t-form-item label="私钥">
              <t-textarea
                v-model="cryptoModel.privateKey"
                :autosize="{ minRows: 5, maxRows: 10 }"
                clearable
              />
            </t-form-item>
          </t-col>
        </t-row>

        <t-row :gutter="[24, 24]">
          <t-col>
            <t-form-item label="源数据">
              <t-textarea v-model="cryptoModel.source" :autosize="{ minRows: 5, maxRows: 10 }"/>
            </t-form-item>
          </t-col>
          <t-col>
            <t-form-item label="加密后数据">
              <t-textarea v-model="cryptoModel.dest" :autosize="{ minRows: 5, maxRows: 10 }"/>
            </t-form-item>
          </t-col>
        </t-row>
      </t-form>
    </div>
  </to-page>
</template>
<script lang="ts" setup>
import ToPage from '@/components/ToPage/index.vue'
import {Encode} from '@/wailsjs/go/crypt/Crypt'
import {crypt} from '@/wailsjs/go/models'
import {MessagePlugin, SelectOption, SelectOptionGroup} from 'tdesign-vue-next'

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

  Encode(crypt.Config.createFrom(toRaw(cryptoModel)))
    .then((dest: any) => {
      cryptoModel.dest = dest as string
    })
    .catch((err: any) => {
      MessagePlugin.error(err)
    })
})
</script>

<style lang="scss" scoped></style>
