#!/bin/bash
# Script de verificación de progreso para Rust Koans

koans=(
    "01_about_variables"
    "02_about_ownership"
    "03_about_structs"
    "04_about_traits"
    "05_about_errors"
    "06_about_collections"
    "07_about_iterators"
    "08_about_lifetimes"
    "09_about_concurrency"
    "10_about_modules"
)

echo "🦀 Verificando progreso de Rust Koans..."
echo ""

completados=0
total=${#koans[@]}

for koan in "${koans[@]}"; do
    if [ -d "$koan" ]; then
        cd "$koan"
        if cargo test 2>&1 | grep -q "test result: ok"; then
            echo "✅ $koan - COMPLETADO"
            ((completados++))
        else
            echo "❌ $koan - Pendiente"
        fi
        cd ..
    fi
done

echo ""
echo "Progreso: $completados/$total koans completados"
porcentaje=$((completados * 100 / total))
echo "Porcentaje: $porcentaje%"
