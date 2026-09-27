const COUNTRY_CODE = '237';

export const OPERATOR_PREFIXES = {
  mtn: ['67', '650', '651', '652', '653', '654', '680', '681', '682', '683'],
  orange: ['69', '640', '641', '642', '655', '656', '657', '658', '659', '686', '687', '688', '689'],
  nexttel: ['66', '684', '685'],
  camtel: ['62', '222', '233', '242', '243'],
} as const;

export type Operator = keyof typeof OPERATOR_PREFIXES;

const NATIONAL_NUMBER = new RegExp(`^(?:(?:\\+|00)?${COUNTRY_CODE})?(\\d{9})$`);

function nationalNumber(tel: string): string | null {
  const stripped = tel.replace(/\s+/g, '');
  const match = stripped.match(NATIONAL_NUMBER);
  return match ? match[1] : null;
}

export function check(tel: string): Operator | null {
  const number = nationalNumber(tel);

  if (number === null) {
    return null;
  }

  for (const operator of Object.keys(OPERATOR_PREFIXES) as Operator[]) {
    for (const prefix of OPERATOR_PREFIXES[operator]) {
      if (number.startsWith(prefix)) {
        return operator;
      }
    }
  }

  return null;
}

export function isMTN(tel: string): boolean {
  return check(tel) === 'mtn';
}

export function isOrange(tel: string): boolean {
  return check(tel) === 'orange';
}

export function isNexttel(tel: string): boolean {
  return check(tel) === 'nexttel';
}

export function isCamtel(tel: string): boolean {
  return check(tel) === 'camtel';
}
