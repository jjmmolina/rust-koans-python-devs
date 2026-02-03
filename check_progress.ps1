# Script de verificación de progreso
$koans = @(
    "01_about_variables",
    "02_about_ownership",
    "03_about_structs",
    "04_about_traits",
    "05_about_errors",
    "06_about_collections",
    "07_about_iterators",
    "08_about_lifetimes",
    "09_about_concurrency",
    "10_about_modules"
)

Write-Host "🦀 Verificando progreso de Rust Koans..." -ForegroundColor Cyan
Write-Host ""

$completados = 0
$total = $koans.Count

foreach ($koan in $koans) {
    if (Test-Path $koan) {
        Push-Location $koan
        $resultado = cargo test 2>&1 | Out-String
        Pop-Location
        
        if ($resultado -match "test result: ok") {
            Write-Host "✅ $koan - COMPLETADO" -ForegroundColor Green
            $completados++
        } else {
            Write-Host "❌ $koan - Pendiente" -ForegroundColor Red
        }
    }
}

Write-Host ""
Write-Host "Progreso: $completados/$total koans completados" -ForegroundColor Yellow
$porcentaje = [math]::Round(($completados / $total) * 100, 2)
Write-Host "Porcentaje: $porcentaje%" -ForegroundColor Yellow
