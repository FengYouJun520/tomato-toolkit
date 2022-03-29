export const fillOptions = [
  {
    label: 'DEFAULT',
    value: 'DEFAULT',
  },
  {
    label: 'INSERT',
    value: 'INSERT',
  },
  {
    label: 'UPDATE',
    value: 'UPDATE',
  },
  {
    label: 'INSERT_UPDATE',
    value: 'INSERT_UPDATE',
  },
]

export interface FillInput {
  key: string
  value: string
  fill: string
}

export const assignIds = [
  {
    label: 'NONE',
    value: 'NONE',
  },
  {
    label: 'INPUT',
    value: 'INPUT',
  },
  {
    label: 'AUTO',
    value: 'AUTO',
  },
  {
    label: 'ASSIGN_ID',
    value: 'ASSIGN_ID',
  },
  {
    label: 'ASSIGN_UUID',
    value: 'ASSIGN_UUID',
    disabled: true,
  },
]
