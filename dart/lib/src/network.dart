/// Identify the Cameroonian mobile operator (MTN, Orange, Nexttel, Camtel) that issued a
/// phone number. See `../spec/README.md` for the input rules and prefix table.
library;

/// A Cameroonian mobile network operator.
enum Operator { mtn, orange, nexttel, camtel }

const String _countryCode = '237';

/// The prefix ranges assigned to each operator. A prefix names the operator that issued the
/// number; portability lets a subscriber keep the number on another network.
const Map<Operator, List<String>> operatorPrefixes = {
  Operator.mtn: [
    '67',
    '650',
    '651',
    '652',
    '653',
    '654',
    '680',
    '681',
    '682',
    '683',
  ],
  Operator.orange: [
    '69',
    '640',
    '641',
    '642',
    '655',
    '656',
    '657',
    '658',
    '659',
    '686',
    '687',
    '688',
    '689',
  ],
  Operator.nexttel: ['66', '684', '685'],
  Operator.camtel: ['62', '222', '233', '242', '243'],
};

final RegExp _nationalNumberPattern = RegExp(
  '^(?:(?:\\+|00)?$_countryCode)?(\\d{9})\$',
);

/// Returns the operator that issued [tel], or `null` if it isn't a valid Cameroonian number.
Operator? check(String tel) {
  final number = _nationalNumber(tel);
  if (number == null) {
    return null;
  }

  for (final entry in operatorPrefixes.entries) {
    if (entry.value.any(number.startsWith)) {
      return entry.key;
    }
  }

  return null;
}

bool isMtn(String tel) => check(tel) == Operator.mtn;

bool isOrange(String tel) => check(tel) == Operator.orange;

bool isNexttel(String tel) => check(tel) == Operator.nexttel;

bool isCamtel(String tel) => check(tel) == Operator.camtel;

/// Strips whitespace and the optional `237`/`+237`/`00237` country code, returning the
/// 9-digit national number, or `null` if what's left isn't exactly 9 digits.
String? _nationalNumber(String tel) {
  final compact = tel.replaceAll(RegExp(r'\s+'), '');
  return _nationalNumberPattern.firstMatch(compact)?.group(1);
}
