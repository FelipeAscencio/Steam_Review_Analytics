//! Este módulo contiene la lógica de los tests para verificar el
//! determinismo de la implementaciòn en multiples escenarios de ejecución.

#[cfg(test)]
mod tests {
    // Imports de funciones/estructuras propias.
    use crate::{preparar_salida_final, procesar_archivo_con_pool};

    /// 'Helper' para obtener el resultado final (`SalidaFinal`) dado un número de hilos.
    /// Este helper facilita la ejecución del procesamiento del archivo con el número de hilos especificado
    /// y luego prepara el resultado final para ser utilizado en los tests.
    fn obtener_salida_final(hilos: usize) -> crate::estadisticas_serializables::SalidaFinal {
        let ruta = "dataset_test".to_string();
        let conteo = procesar_archivo_con_pool(ruta, hilos);
        preparar_salida_final(&conteo)
    }

    /// 'Test' para verificar los resultados con diferentes números de hilos.
    /// En esta prueba, se ejecuta la función `obtener_salida_final` con diferentes números de hilos
    /// (1, 4, 8, 16) y se comparan los resultados. La prueba asegura que los resultados obtenidos sean
    /// iguales, verificando tanto los juegos como las reseñas para asegurar el determinismo del programa.
    #[test]
    fn test_resultados_con_diferentes_hilos() {
        let hilos_a_probar = vec![1, 4, 8, 16];
        let mut resultados = Vec::new();
        for &hilos in &hilos_a_probar {
            let salida = obtener_salida_final(hilos);
            resultados.push((hilos, salida));
        }

        let (_, ref base) = resultados[0];

        // Se iteran todos los resultados obtenidos.
        for (_hilos, resultado) in resultados.iter().skip(1) {
            // Se comparan el top 3 juegos obtenidos.
            for (juego_base, juego_resultado) in base.top_games.iter().zip(&resultado.top_games) {
                assert_eq!(juego_base.game, juego_resultado.game);
                assert_eq!(juego_base.review_count, juego_resultado.review_count);

                // Se comparan las 3 mejores reseñas del mejor juego.
                if juego_base == &base.top_games[0] {
                    let top_reviews_base = &juego_base.languages;
                    let top_reviews_resultado = &juego_resultado.languages;
                    let top_reviews_base = top_reviews_base.iter().take(3);
                    let top_reviews_resultado = top_reviews_resultado.iter().take(3);
                    for (idioma_base, idioma_resultado) in
                        top_reviews_base.zip(top_reviews_resultado)
                    {
                        assert_eq!(idioma_base.language, idioma_resultado.language);
                        assert_eq!(idioma_base.review_count, idioma_resultado.review_count);
                        assert_eq!(idioma_base.top_review, idioma_resultado.top_review);
                        assert_eq!(
                            idioma_base.top_review_votes,
                            idioma_resultado.top_review_votes
                        );
                    }
                }
            }

            // Se compara el top 3 idiomas obtenidos.
            for (idioma_base, idioma_resultado) in
                base.top_languages.iter().zip(&resultado.top_languages)
            {
                assert_eq!(idioma_base.language, idioma_resultado.language);
                assert_eq!(idioma_base.review_count, idioma_resultado.review_count);
                for (review_base, review_resultado) in idioma_base
                    .top_reviews
                    .iter()
                    .zip(&idioma_resultado.top_reviews)
                {
                    assert_eq!(review_base.review, review_resultado.review);
                    assert_eq!(review_base.votes, review_resultado.votes);
                }
            }
        }
    }

    /// Prueba para verificar la repetibilidad de los resultados con el mismo número de hilos.
    ///
    /// Esta prueba asegura que al ejecutar la función `obtener_salida_final` varias veces con el mismo
    /// número de hilos (en este caso 3), los resultados sean iguales en todas las repeticiones.
    #[test]
    fn test_repetibilidad_con_mismos_hilos() {
        let hilos = 3;
        let repeticiones = 5;
        let mut resultados = Vec::new();
        for _i in 0..repeticiones {
            let salida = obtener_salida_final(hilos);
            resultados.push((_i, salida));
        }

        let (_, ref base) = resultados[0];

        // Se iteran todos los resultados obtenidos.
        for (_i, resultado) in resultados.iter().skip(1) {
            // Se comparan el top 3 juegos obtenidos.
            for (juego_base, juego_resultado) in base.top_games.iter().zip(&resultado.top_games) {
                assert_eq!(juego_base.game, juego_resultado.game);
                assert_eq!(juego_base.review_count, juego_resultado.review_count);

                // Se comparan las 3 mejores reseñas del mejor juego.
                if juego_base == &base.top_games[0] {
                    let top_reviews_base = &juego_base.languages;
                    let top_reviews_resultado = &juego_resultado.languages;
                    let top_reviews_base = top_reviews_base.iter().take(3);
                    let top_reviews_resultado = top_reviews_resultado.iter().take(3);
                    for (idioma_base, idioma_resultado) in
                        top_reviews_base.zip(top_reviews_resultado)
                    {
                        assert_eq!(idioma_base.language, idioma_resultado.language);
                        assert_eq!(idioma_base.review_count, idioma_resultado.review_count);
                        assert_eq!(idioma_base.top_review, idioma_resultado.top_review);
                        assert_eq!(
                            idioma_base.top_review_votes,
                            idioma_resultado.top_review_votes
                        );
                    }
                }
            }

            // Se compara el top 3 idiomas obtenidos.
            for (idioma_base, idioma_resultado) in
                base.top_languages.iter().zip(&resultado.top_languages)
            {
                assert_eq!(idioma_base.language, idioma_resultado.language);
                assert_eq!(idioma_base.review_count, idioma_resultado.review_count);
                for (review_base, review_resultado) in idioma_base
                    .top_reviews
                    .iter()
                    .zip(&idioma_resultado.top_reviews)
                {
                    assert_eq!(review_base.review, review_resultado.review);
                    assert_eq!(review_base.votes, review_resultado.votes);
                }
            }
        }
    }
}
