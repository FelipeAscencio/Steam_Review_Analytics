//! Este módulo contiene la lógica de las estadísticas internas del programa.

// Imports de crates externas.
use std::collections::HashMap;

// Constantes.
const MAXIMA_CANTIDAD_REVIEWS: usize = 10;

/// Estructura que guarda la información global de:
/// - Juegos.
/// - Idiomas.
#[derive(Debug, Default)]
pub struct EstadisticasGlobales {
    pub juegos: HashMap<String, InfoJuego>,
    pub por_idioma: HashMap<String, InfoIdioma>,
}

/// Estructura que guarda la información de un juego procesado.
#[derive(Debug, Default)]
pub struct InfoJuego {
    pub cantidad_total: usize,
    pub por_idioma: HashMap<String, usize>,
    pub mejores_reviews: HashMap<String, (String, u32)>,
}

/// Estructura que guarda la información de un idioma procesado.
#[derive(Debug, Default)]
pub struct InfoIdioma {
    pub cantidad_total: usize,
    pub top_reviews: Vec<(String, u32)>,
}

/// Métodos de mergeo de las estadísticas obtenidas.
impl EstadisticasGlobales {
    /// Método para fusionar estadísticas parciales en una global.
    pub fn merge_into(&self, destino: &mut EstadisticasGlobales) {
        self.merge_juegos(destino);
        self.merge_idiomas(destino);
    }

    /// Fusiona la información de juegos de `self` en `destino`.
    fn merge_juegos(&self, destino: &mut EstadisticasGlobales) {
        for (juego, info) in &self.juegos {
            let entry = destino.juegos.entry(juego.clone()).or_default();
            entry.cantidad_total += info.cantidad_total;
            for (idioma, count) in &info.por_idioma {
                *entry.por_idioma.entry(idioma.clone()).or_insert(0) += *count;
            }

            for (idioma, (texto, votos)) in &info.mejores_reviews {
                match entry.mejores_reviews.entry(idioma.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        if *votos > e.get().1 {
                            e.insert((texto.clone(), *votos));
                        }
                    }

                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert((texto.clone(), *votos));
                    }
                }
            }
        }
    }

    /// Fusiona la información de idiomas de `self` en `destino`.
    fn merge_idiomas(&self, destino: &mut EstadisticasGlobales) {
        for (idioma, info) in &self.por_idioma {
            let entry = destino.por_idioma.entry(idioma.clone()).or_default();
            entry.cantidad_total += info.cantidad_total;
            entry.top_reviews.extend(info.top_reviews.clone());
            entry
                .top_reviews
                .sort_by_key(|(_, votos)| std::cmp::Reverse(*votos));
            entry.top_reviews.truncate(MAXIMA_CANTIDAD_REVIEWS);
        }
    }
}

/// Se utiliza en la función de procesamiento de chunks.
pub type EstadisticasParciales = EstadisticasGlobales;
