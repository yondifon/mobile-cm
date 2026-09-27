<?php

namespace Malico\MobileCM;

class Network
{
    const PREFIX = '237';

    // The operator each range was assigned to. Portability lets a subscriber keep the number on another network.
    const OPERATOR_PREFIXES = [
        'mtn' => ['67', '650', '651', '652', '653', '654', '680', '681', '682', '683'],
        'orange' => ['69', '640', '641', '642', '655', '656', '657', '658', '659', '686', '687', '688', '689'],
        'nexttel' => ['66', '684', '685'],
        'camtel' => ['62', '222', '233', '242', '243'],
    ];

    public static function isMTN(string $tel): bool
    {
        return self::check($tel) === 'mtn';
    }

    public static function isOrange(string $tel): bool
    {
        return self::check($tel) === 'orange';
    }

    public static function isNexttel(string $tel): bool
    {
        return self::check($tel) === 'nexttel';
    }

    public static function isCamtel(string $tel): bool
    {
        return self::check($tel) === 'camtel';
    }

    public static function check(string $tel): ?string
    {
        $number = self::nationalNumber($tel);

        if ($number === null) {
            return null;
        }

        foreach (self::OPERATOR_PREFIXES as $operator => $prefixes) {
            foreach ($prefixes as $prefix) {
                if (str_starts_with($number, $prefix)) {
                    return $operator;
                }
            }
        }

        return null;
    }

    private static function nationalNumber(string $tel): ?string
    {
        // Unicode White_Space, spelled out because PCRE's \s misses some; null on invalid UTF-8.
        $tel = preg_replace('/[\t\n\x{0B}\f\r \x{85}\x{A0}\x{1680}\x{2000}-\x{200A}\x{2028}\x{2029}\x{202F}\x{205F}\x{3000}]+/u', '', $tel) ?? '';

        // "+237 6 77 12 34 56" → "677123456"
        return preg_match('/^(?:(?:\+|00)?' . self::PREFIX . ')?(\d{9})$/', $tel, $match) ? $match[1] : null;
    }
}
